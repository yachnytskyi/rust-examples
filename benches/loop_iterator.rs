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

#[inline(never)]
fn filter_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in (0..n).filter(|&x| x % 2 == 0) {
        sum += i;
    }
    sum
}

#[inline(never)]
fn zip_for_loop(n: u64) -> u64 {
    let v: Vec<u64> = (0..n).collect();
    let mut sum = 0;
    for (a, b) in (0..n).zip(v.iter()) {
        sum += a + *b;
    }
    sum
}

#[inline(never)]
fn skip_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in (0..n).skip(10) {
        sum += i;
    }
    sum
}

#[inline(never)]
fn take_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in (0..n).take(1000) {
        sum += i;
    }
    sum
}

#[inline(never)]
fn flat_map_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in (0..n).flat_map(|x| vec![x, x + 1]) {
        sum += i;
    }
    sum
}

#[inline(never)]
fn any_for_loop(n: u64) -> bool {
    (0..n).any(|x| x == 100)
}

#[inline(never)]
fn all_for_loop(n: u64) -> bool {
    (0..n).all(|x| x < n)
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

    // Additional benchmark functions

    c.bench_function("filter_for_loop", |b| {
        b.iter(|| black_box(filter_for_loop(n)))
    });

    c.bench_function("zip_for_loop", |b| b.iter(|| black_box(zip_for_loop(n))));

    c.bench_function("skip_for_loop", |b| b.iter(|| black_box(skip_for_loop(n))));

    c.bench_function("take_for_loop", |b| b.iter(|| black_box(take_for_loop(n))));

    c.bench_function("flat_map_for_loop", |b| {
        b.iter(|| black_box(flat_map_for_loop(n)))
    });

    c.bench_function("any_for_loop", |b| b.iter(|| black_box(any_for_loop(n))));

    c.bench_function("all_for_loop", |b| b.iter(|| black_box(all_for_loop(n))));
}

criterion_group!(benches, bench_for_loops);
criterion_main!(benches);
