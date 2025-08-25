use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};

fn make_and_fill_prealloc<const N: usize>() {
    // Allocate with exact capacity so pushes won't reallocate.
    let mut v: Vec<u32> = Vec::with_capacity(N);
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
    // drop at end
}

fn make_and_fill_growing<const N: usize>() {
    // Start with empty Vec so geometric growth + realloc/memcpy cost is measured.
    let mut v: Vec<u32> = Vec::new();
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
    // drop at end
}

fn bench_make_and_fill_for<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("vec_make_and_fill_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    g.bench_function("Vec::with_capacity(N) then push N", |b| {
        b.iter(|| make_and_fill_prealloc::<N>())
    });

    g.bench_function("Vec::new() then push N (growing)", |b| {
        b.iter(|| make_and_fill_growing::<N>())
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_make_and_fill_for::<10>(c);
    bench_make_and_fill_for::<100>(c);
    bench_make_and_fill_for::<1_000>(c);
    bench_make_and_fill_for::<10_000>(c);
    bench_make_and_fill_for::<1_000_000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
