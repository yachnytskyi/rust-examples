use criterion::{Criterion, black_box, criterion_group, criterion_main};

// ==== Benchmark functions for different loops ====

#[inline(never)]
fn basic_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

#[inline(never)]
fn iterator_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

#[inline(never)]
fn map_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in (0..n).map(|x| x) {
        sum += i;
    }
    sum
}

#[inline(never)]
fn fold_for_loop(n: u64) -> u64 {
    (0..n).fold(0, |acc, x| acc + x)
}

#[inline(never)]
fn iterator_map_collect(n: u64) -> u64 {
    (0..n).map(|x| x).collect::<Vec<u64>>().iter().sum()
}

#[inline(never)]
fn for_each_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    (0..n).for_each(|x| sum += x);
    sum
}

// ==== Benchmark group ====

fn bench_for_loops(c: &mut Criterion) {
    let n = 1_000_000;

    c.bench_function("basic_for_loop", |b| {
        b.iter(|| black_box(basic_for_loop(n)))
    });

    c.bench_function("iterator_for_loop", |b| {
        b.iter(|| black_box(iterator_for_loop(n)))
    });

    c.bench_function("map_for_loop", |b| b.iter(|| black_box(map_for_loop(n))));

    c.bench_function("fold_for_loop", |b| b.iter(|| black_box(fold_for_loop(n))));

    c.bench_function("iterator_map_collect", |b| {
        b.iter(|| black_box(iterator_map_collect(n)))
    });

    c.bench_function("for_each_for_loop", |b| {
        b.iter(|| black_box(for_each_for_loop(n)))
    });
}

criterion_group!(benches, bench_for_loops);
criterion_main!(benches);
