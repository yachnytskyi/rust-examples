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
    v.clear();
    if v.capacity() < N {
        v.reserve_exact(N - v.capacity());
    }
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
}

#[inline(never)]
fn fill_smallvec<const N: usize>(v: &mut SmallVec<SmallInline>) {
    v.clear();
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
}

#[inline(never)]
fn fill_tinyvec<const N: usize>(v: &mut TinyVec<[u32; TINY_INLINE]>) {
    v.clear();
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
}

// ------------------------------------------------------------
// Functions under test (Vec)
// ------------------------------------------------------------

#[inline(never)]
fn make_vec_value<const N: usize>() -> Vec<u32> {
    let mut v = Vec::with_capacity(N);
    fill_vec::<N>(&mut v);
    v
}

#[inline(never)]
fn make_vec_box<const N: usize>() -> Box<[u32]> {
    let mut v = Vec::with_capacity(N);
    fill_vec::<N>(&mut v);
    v.into_boxed_slice()
}

// ------------------------------------------------------------
// Functions under test (SmallVec)
// ------------------------------------------------------------

#[inline(never)]
fn make_smallvec_value<const N: usize>() -> SmallVec<SmallInline> {
    let inline = <SmallInline as Array>::size();
    let mut sv: SmallVec<SmallInline> = if N > inline {
        SmallVec::with_capacity(N) // heap-spilled
    } else {
        SmallVec::new() // inline
    };
    fill_smallvec::<N>(&mut sv);
    sv
}

#[inline(never)]
fn make_smallvec_box<const N: usize>() -> Box<[u32]> {
    // Build SmallVec, then convert to Vec → Box.
    // If inline, this will allocate & copy; if spilled, it reuses the buffer.
    let sv = make_smallvec_value::<N>();
    sv.into_vec().into_boxed_slice()
}

// ------------------------------------------------------------
// Functions under test (TinyVec)
// ------------------------------------------------------------

#[inline(never)]
fn make_tinyvec_value<const N: usize>() -> TinyVec<[u32; TINY_INLINE]> {
    let mut tv: TinyVec<[u32; TINY_INLINE]> = if N > TINY_INLINE {
        TinyVec::with_capacity(N) // heap
    } else {
        TinyVec::new() // inline
    };
    fill_tinyvec::<N>(&mut tv);
    tv
}

#[inline(never)]
fn make_tinyvec_box<const N: usize>() -> Box<[u32]> {
    // Build TinyVec, then convert to Vec → Box.
    // Inline → alloc+copy; spilled → reuse.
    let tv = make_tinyvec_value::<N>();
    tv.into_vec().into_boxed_slice()
}

// ------------------------------------------------------------
// Benches
// ------------------------------------------------------------

fn bench_return_forms_for<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("return_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    // ---------- Vec ----------
    g.bench_function("Vec (return by value)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let v = make_vec_value::<N>();
                black_box(v.last().copied());
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("Vec -> Box<[u32]>", |b| {
        b.iter_batched(
            || (),
            |_| {
                let bx = make_vec_box::<N>();
                black_box(bx.len());
            },
            BatchSize::SmallInput,
        )
    });

    // ---------- SmallVec ----------
    g.bench_function("SmallVec (return by value)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let sv = make_smallvec_value::<N>();
                black_box(sv.last().copied());
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("SmallVec -> Box<[u32]>", |b| {
        b.iter_batched(
            || (),
            |_| {
                let bx = make_smallvec_box::<N>();
                black_box(bx.len());
            },
            BatchSize::SmallInput,
        )
    });

    // ---------- TinyVec ----------
    g.bench_function("TinyVec (return by value)", |b| {
        b.iter_batched(
            || (),
            |_| {
                let tv = make_tinyvec_value::<N>();
                black_box(tv.last().copied());
            },
            BatchSize::SmallInput,
        )
    });

    g.bench_function("TinyVec -> Box<[u32]>", |b| {
        b.iter_batched(
            || (),
            |_| {
                let bx = make_tinyvec_box::<N>();
                black_box(bx.len());
            },
            BatchSize::SmallInput,
        )
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_return_forms_for::<10>(c);
    bench_return_forms_for::<100>(c);
    bench_return_forms_for::<1_000>(c);
    bench_return_forms_for::<10_000>(c);
    bench_return_forms_for::<10_0000>(c);

    bench_return_forms_for::<1_000_000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
