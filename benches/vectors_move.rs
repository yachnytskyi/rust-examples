use criterion::{BatchSize, Criterion, Throughput, black_box, criterion_group, criterion_main};
use smallvec::{Array, SmallVec};
use tinyvec::TinyVec;

// ------------------------------------------------------------
// Common types & helpers
// ------------------------------------------------------------

type SmallInline = [u32; 128];
const TINY_INLINE: usize = 100;

#[inline(never)]
fn fill_vec<const N: usize>(v: &mut Vec<u32>) {
    v.clear(); // keep capacity (no alloc)
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
}

#[inline(never)]
fn fill_smallvec<const N: usize>(v: &mut SmallVec<SmallInline>) {
    v.clear(); // keep mode & capacity
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
}

#[inline(never)]
fn fill_tinyvec<const N: usize>(v: &mut TinyVec<[u32; TINY_INLINE]>) {
    v.clear(); // keep mode & capacity
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
}

// ------------------------------------------------------------
// Owned-move, zero-alloc per iteration (Vec / SmallVec / TinyVec)
// Repos keep two preallocated buffers and use a swap+recycle scheme.
// ------------------------------------------------------------

struct VecRepoMove<const N: usize> {
    buf: Vec<u32>,
    spare: Vec<u32>,
}
impl<const N: usize> VecRepoMove<N> {
    fn new() -> Self {
        Self {
            buf: Vec::with_capacity(N),
            spare: Vec::with_capacity(N),
        }
    }
    #[inline(never)]
    fn fetch_move(&mut self) -> Vec<u32> {
        fill_vec::<N>(&mut self.buf);
        // Move filled buffer out without losing a preallocated one:
        core::mem::swap(&mut self.buf, &mut self.spare);
        core::mem::take(&mut self.spare) // filled Vec (cap N); spare becomes empty (cap 0) until recycle
    }
    #[inline(never)]
    fn recycle(&mut self, mut v: Vec<u32>) {
        v.clear();
        self.spare = v; // return capacity to repo
    }
}

struct SmallVecRepoMove<const N: usize> {
    buf: SmallVec<SmallInline>,
    spare: SmallVec<SmallInline>,
}
impl<const N: usize> SmallVecRepoMove<N> {
    fn new() -> Self {
        let inline = <SmallInline as Array>::size();
        let (buf, spare) = if N > inline {
            (SmallVec::with_capacity(N), SmallVec::with_capacity(N))
        } else {
            (SmallVec::new(), SmallVec::new()) // inline; zero alloc path
        };
        Self { buf, spare }
    }
    #[inline(never)]
    fn fetch_move(&mut self) -> SmallVec<SmallInline> {
        fill_smallvec::<N>(&mut self.buf);
        core::mem::swap(&mut self.buf, &mut self.spare);
        core::mem::take(&mut self.spare)
    }
    #[inline(never)]
    fn recycle(&mut self, mut v: SmallVec<SmallInline>) {
        v.clear();
        self.spare = v;
    }
}

struct TinyVecRepoMove<const N: usize> {
    buf: TinyVec<[u32; TINY_INLINE]>,
    spare: TinyVec<[u32; TINY_INLINE]>,
}
impl<const N: usize> TinyVecRepoMove<N> {
    fn new() -> Self {
        let (buf, spare) = if N > TINY_INLINE {
            (
                TinyVec::<[u32; TINY_INLINE]>::with_capacity(N),
                TinyVec::<[u32; TINY_INLINE]>::with_capacity(N),
            )
        } else {
            (TinyVec::new(), TinyVec::new())
        };
        Self { buf, spare }
    }
    #[inline(never)]
    fn fetch_move(&mut self) -> TinyVec<[u32; TINY_INLINE]> {
        fill_tinyvec::<N>(&mut self.buf);
        core::mem::swap(&mut self.buf, &mut self.spare);
        core::mem::take(&mut self.spare)
    }
    #[inline(never)]
    fn recycle(&mut self, mut v: TinyVec<[u32; TINY_INLINE]>) {
        v.clear();
        self.spare = v;
    }
}

// ------------------------------------------------------------
// Box<[u32]> zero-alloc per iteration
// Vec -> Box is naturally zero-alloc when len == cap (we prealloc).
// SmallVec/TinyVec: force heap (even for small N) by using a preallocated Vec
// buffer and constructing the tiny/small vector from that Vec each iter.
// Boxing then reuses the same heap buffer; recycle converts Box back to Vec.
// ------------------------------------------------------------

struct VecRepoBox<const N: usize> {
    buf: Vec<u32>,
    spare: Vec<u32>,
}
impl<const N: usize> VecRepoBox<N> {
    fn new() -> Self {
        Self {
            buf: Vec::with_capacity(N),
            spare: Vec::with_capacity(N),
        }
    }
    #[inline(never)]
    fn fetch_box(&mut self) -> Box<[u32]> {
        fill_vec::<N>(&mut self.buf);
        core::mem::swap(&mut self.buf, &mut self.spare);
        let out_vec = core::mem::take(&mut self.spare);
        out_vec.into_boxed_slice() // zero-alloc boxing
    }
    #[inline(never)]
    fn recycle_box(&mut self, bx: Box<[u32]>) {
        let mut v = bx.into_vec(); // reuses buffer
        v.clear();
        self.spare = v;
    }
}

struct SmallVecRepoBox<const N: usize> {
    // We keep heap Vec buffers to *force* spilled mode for SmallVec even when N <= inline.
    buf_v: Vec<u32>,
    spare_v: Vec<u32>,
}
impl<const N: usize> SmallVecRepoBox<N> {
    fn new() -> Self {
        Self {
            buf_v: Vec::with_capacity(N),
            spare_v: Vec::with_capacity(N),
        }
    }
    #[inline(never)]
    fn fetch_box(&mut self) -> Box<[u32]> {
        // Ensure we take a preallocated Vec for this iteration
        core::mem::swap(&mut self.buf_v, &mut self.spare_v);
        let v_in = core::mem::take(&mut self.buf_v);
        // Construct SmallVec *spilled* from Vec (no alloc), fill via SmallVec's push path.
        let mut sv: SmallVec<SmallInline> = SmallVec::from_vec(v_in);
        fill_smallvec::<N>(&mut sv);
        let v_out = sv.into_vec(); // no alloc
        v_out.into_boxed_slice() // no alloc
    }
    #[inline(never)]
    fn recycle_box(&mut self, bx: Box<[u32]>) {
        let mut v = bx.into_vec();
        v.clear();
        self.buf_v = v; // will be swapped into place next fetch
    }
}

struct TinyVecRepoBox<const N: usize> {
    // Keep heap Vec buffers to force spilled mode for TinyVec even for small N.
    buf_v: Vec<u32>,
    spare_v: Vec<u32>,
}
impl<const N: usize> TinyVecRepoBox<N> {
    fn new() -> Self {
        Self {
            buf_v: Vec::with_capacity(N),
            spare_v: Vec::with_capacity(N),
        }
    }
    #[inline(never)]
    fn fetch_box(&mut self) -> Box<[u32]> {
        core::mem::swap(&mut self.buf_v, &mut self.spare_v);
        let v_in = core::mem::take(&mut self.buf_v);
        // Force heap mode by wrapping the preallocated Vec directly.
        let mut tv: TinyVec<[u32; TINY_INLINE]> = TinyVec::Heap(v_in);
        fill_tinyvec::<N>(&mut tv);
        let v_out = tv.into_vec(); // takes the Vec without reallocation
        v_out.into_boxed_slice() // no alloc
    }
    #[inline(never)]
    fn recycle_box(&mut self, bx: Box<[u32]>) {
        let mut v = bx.into_vec();
        v.clear();
        self.buf_v = v;
    }
}

// ------------------------------------------------------------
// Benches
// ------------------------------------------------------------

fn bench_owned_moves_for<const N: usize>(c: &mut Criterion) {
    let mut vec_repo = VecRepoMove::<N>::new();
    let mut small_repo = SmallVecRepoMove::<N>::new();
    let mut tiny_repo = TinyVecRepoMove::<N>::new();

    let mut g = c.benchmark_group(format!("owned_move_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    g.bench_function("Vec (move, noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let mut v = vec_repo.fetch_move();
                black_box(v.last().copied());
                v.clear();
                vec_repo.recycle(v);
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("SmallVec (move, noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let mut v = small_repo.fetch_move();
                black_box(v.last().copied());
                v.clear();
                small_repo.recycle(v);
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("TinyVec (move, noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let mut v = tiny_repo.fetch_move();
                black_box(v.last().copied());
                v.clear();
                tiny_repo.recycle(v);
            },
            BatchSize::SmallInput,
        )
    });

    g.finish();
}

fn bench_box_moves_for<const N: usize>(c: &mut Criterion) {
    let mut vec_repo = VecRepoBox::<N>::new();
    let mut small_repo = SmallVecRepoBox::<N>::new();
    let mut tiny_repo = TinyVecRepoBox::<N>::new();

    let mut g = c.benchmark_group(format!("box_move_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    g.bench_function("Vec -> Box<[u32]> (noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let bx = vec_repo.fetch_box();
                black_box(bx.len());
                vec_repo.recycle_box(bx);
            },
            BatchSize::SmallInput,
        )
    });

    // Note: For SmallVec/TinyVec we force spilled mode via Vec buffers to make boxing zero-alloc.
    g.bench_function("SmallVec -> Box<[u32]> (noalloc, forced-heap)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let bx = small_repo.fetch_box();
                black_box(bx.len());
                small_repo.recycle_box(bx);
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("TinyVec -> Box<[u32]> (noalloc, forced-heap)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let bx = tiny_repo.fetch_box();
                black_box(bx.len());
                tiny_repo.recycle_box(bx);
            },
            BatchSize::SmallInput,
        )
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_owned_moves_for::<10>(c);
    bench_box_moves_for::<10>(c);

    bench_owned_moves_for::<100>(c);
    bench_box_moves_for::<100>(c);

    bench_owned_moves_for::<1_000>(c);
    bench_box_moves_for::<1_000>(c);

    bench_owned_moves_for::<10_000>(c);
    bench_box_moves_for::<10_000>(c);

    bench_owned_moves_for::<1_000_000>(c);
    bench_box_moves_for::<1_000_000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
