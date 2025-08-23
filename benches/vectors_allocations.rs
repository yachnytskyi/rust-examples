use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use smallvec::SmallVec;
use tinyvec::TinyVec;

// Inline capacities used by SmallVec and TinyVec in these tests.
type SmallInline = [u32; 128];
const TINY_INLINE: usize = 100;

// -----------------------------
// Constructors + fill helpers
// -----------------------------

#[inline(never)]
fn make_and_fill_vec_prealloc<const N: usize>() {
    let mut v: Vec<u32> = Vec::with_capacity(N); // preallocate exactly N
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn make_and_fill_vec_grow<const N: usize>() {
    let mut v: Vec<u32> = Vec::new(); // grow geometrically
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn make_and_fill_smallvec_prealloc<const N: usize>() {
    // If N <= inline cap, this stays inline; if N > inline cap, spills with exact heap cap.
    let mut v: SmallVec<SmallInline> = SmallVec::with_capacity(N);
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn make_and_fill_smallvec_grow<const N: usize>() {
    // Starts inline; will spill + grow if N > inline cap.
    let mut v: SmallVec<SmallInline> = SmallVec::new();
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn make_and_fill_tinyvec_prealloc<const N: usize>() {
    // If N <= TINY_INLINE, remains inline; else allocates exact heap cap N.
    let mut v: TinyVec<[u32; TINY_INLINE]> = TinyVec::with_capacity(N);
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn make_and_fill_tinyvec_grow<const N: usize>() {
    // Starts inline; spills + grows when N > TINY_INLINE.
    let mut v: TinyVec<[u32; TINY_INLINE]> = TinyVec::new();
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

// -----------------------------
// Bench harness
// -----------------------------

fn bench_make_and_fill_all_for<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("make_and_fill_all_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    // Vec
    g.bench_function("Vec::with_capacity(N) + push N", |b| {
        b.iter(|| make_and_fill_vec_prealloc::<N>())
    });
    g.bench_function("Vec::new() + push N (growing)", |b| {
        b.iter(|| make_and_fill_vec_grow::<N>())
    });

    // SmallVec
    g.bench_function("SmallVec::with_capacity(N) + push N", |b| {
        b.iter(|| make_and_fill_smallvec_prealloc::<N>())
    });
    g.bench_function("SmallVec::new() + push N (growing/inline→spill)", |b| {
        b.iter(|| make_and_fill_smallvec_grow::<N>())
    });

    // TinyVec
    g.bench_function("TinyVec::with_capacity(N) + push N", |b| {
        b.iter(|| make_and_fill_tinyvec_prealloc::<N>())
    });
    g.bench_function("TinyVec::new() + push N (growing/inline→spill)", |b| {
        b.iter(|| make_and_fill_tinyvec_grow::<N>())
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_make_and_fill_all_for::<10>(c);
    bench_make_and_fill_all_for::<100>(c);
    bench_make_and_fill_all_for::<1_000>(c);
    bench_make_and_fill_all_for::<10_000>(c);
    bench_make_and_fill_all_for::<1_000_000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
