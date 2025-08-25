use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use small_map::SmallMap;
use std::collections::HashMap;
use tinymap::ArrayMap;

// How many entries SmallMap keeps inline before spilling.
const SMALLMAP_INLINE: usize = 128;

// -----------------------------
// Make & fill (alloc + insert are timed)
// -----------------------------

fn make_and_fill_hashmap_prealloc<const N: usize>() {
    // Pre-allocate to avoid rehash growth noise.
    let mut m: HashMap<u32, u32> = HashMap::with_capacity(N);
    for i in 0..N as u32 {
        m.insert(black_box(i), black_box(i));
    }
    black_box(m.len());
    black_box(m.get(&0));
}

fn make_and_fill_smallmap_inline_then_spill<const N: usize>() {
    // Inline up to SMALLMAP_INLINE; spills to a heap-backed map beyond that.
    let mut m: SmallMap<SMALLMAP_INLINE, u32, u32> = SmallMap::new();
    for i in 0..N as u32 {
        m.insert(black_box(i), black_box(i));
    }
    black_box(m.len());
    black_box(m.get(&0));
}

fn make_and_fill_arraymap_stack<const N: usize>() {
    // Fixed capacity on the stack, exactly N; ensure your "small Ns" never exceed what you want on the stack.
    let mut m: ArrayMap<u32, u32, N> = ArrayMap::new();
    for i in 0..N as u32 {
        // Capacity == N, so inserting N items will not overflow.
        m.insert(black_box(i), black_box(i));
    }
    black_box(m.len());
    black_box(m.get(&0));
}

// -----------------------------
// Bench harnesses
// -----------------------------

fn bench_make_and_fill_small_ns<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("map_make_and_fill_small_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    g.bench_function("HashMap::with_capacity(N) + insert N", |b| {
        b.iter(|| make_and_fill_hashmap_prealloc::<N>())
    });
    g.bench_function("SmallMap::new() + insert N", |b| {
        b.iter(|| make_and_fill_smallmap_inline_then_spill::<N>())
    });
    g.bench_function("ArrayMap::new() + insert N (stack, fixed)", |b| {
        b.iter(|| make_and_fill_arraymap_stack::<N>())
    });

    g.finish();
}

fn bench_make_and_fill_large_ns<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("map_make_and_fill_large_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    g.bench_function("HashMap::with_capacity(N) + insert N", |b| {
        b.iter(|| make_and_fill_hashmap_prealloc::<N>())
    });
    g.bench_function("SmallMap::new() + insert N (may spill)", |b| {
        b.iter(|| make_and_fill_smallmap_inline_then_spill::<N>())
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    // Keep these "small" so ArrayMap’s stack footprint stays reasonable.
    bench_make_and_fill_small_ns::<5>(c);
    bench_make_and_fill_small_ns::<10>(c);
    bench_make_and_fill_small_ns::<14>(c);
    bench_make_and_fill_small_ns::<15>(c);
    bench_make_and_fill_small_ns::<20>(c);
    bench_make_and_fill_small_ns::<30>(c);
    bench_make_and_fill_small_ns::<40>(c);
    bench_make_and_fill_small_ns::<64>(c);
    bench_make_and_fill_small_ns::<100>(c);
    bench_make_and_fill_small_ns::<128>(c); // matches SMALLMAP_INLINE

    // Larger sizes: compare HashMap vs SmallMap (ArrayMap omitted intentionally).
    bench_make_and_fill_large_ns::<1_000>(c);
    bench_make_and_fill_large_ns::<10_000>(c);
    bench_make_and_fill_large_ns::<100_000>(c);
    bench_make_and_fill_large_ns::<1_000_000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
