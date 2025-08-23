use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use smallvec::SmallVec;
use tinyvec::TinyVec;

// Inline capacities
type SmallInline = [u32; 128];
const TINY_INLINE: usize = 100;

// -----------------------------
// Make & fill (alloc + push are timed)
// -----------------------------

#[inline(never)]
fn make_and_fill_vec_prealloc<const N: usize>() {
    let mut v: Vec<u32> = Vec::with_capacity(N);
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn make_and_fill_smallvec_prealloc<const N: usize>() {
    // Inline if N <= 128; else heap with requested capacity
    let mut v: SmallVec<SmallInline> = SmallVec::with_capacity(N);
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn make_and_fill_tinyvec_prealloc<const N: usize>() {
    // Inline if N <= 100; else heap with requested capacity
    let mut v: TinyVec<[u32; TINY_INLINE]> = TinyVec::with_capacity(N);
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

// -----------------------------
// Bench harness
// -----------------------------

fn bench_make_and_fill_for<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("make_and_fill_prealloc_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    g.bench_function("Vec::with_capacity(N) + push N", |b| {
        b.iter(|| make_and_fill_vec_prealloc::<N>())
    });
    g.bench_function("SmallVec::with_capacity(N) + push N", |b| {
        b.iter(|| make_and_fill_smallvec_prealloc::<N>())
    });
    g.bench_function("TinyVec::with_capacity(N) + push N", |b| {
        b.iter(|| make_and_fill_tinyvec_prealloc::<N>())
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_make_and_fill_for::<10>(c);
    bench_make_and_fill_for::<15>(c);
    bench_make_and_fill_for::<100>(c);
    bench_make_and_fill_for::<1_000>(c);
    bench_make_and_fill_for::<10_000>(c);
    bench_make_and_fill_for::<1_000_000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
