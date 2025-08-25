use criterion::{Criterion, black_box, criterion_group, criterion_main};

// ==== Basic Loop ====

fn basic_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

// ==== Map ====

fn map_without_sugar(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        let x = i;
        sum += x;
    }
    sum
}

fn map_for_loop(n: u64) -> u64 {
    (0..n).map(|x| x).sum()
}

// ==== Fold ====

fn fold_without_sugar(n: u64) -> u64 {
    let mut acc = 0;
    for i in 0..n {
        acc += i;
    }
    acc
}

fn fold_for_loop(n: u64) -> u64 {
    (0..n).fold(0, |acc, x| acc + x)
}

// ==== Iterator Map Collect ====

fn iterator_map_collect_without_sugar(n: u64) -> u64 {
    let mut sum = 0;
    let mut vec = Vec::with_capacity(n as usize);
    for i in 0..n {
        vec.push(i);
    }
    for val in vec.iter() {
        sum += val;
    }
    sum
}

fn iterator_map_collect(n: u64) -> u64 {
    (0..n).map(|x| x).collect::<Vec<u64>>().iter().sum()
}

// ==== For Each ====

fn for_each_without_sugar(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
    }
    sum
}

fn for_each_for_loop(n: u64) -> u64 {
    let mut sum = 0;
    (0..n).for_each(|x| sum += x);
    sum
}

// ==== Filter ====

fn filter_without_sugar(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        if i % 2 == 0 {
            sum += i;
        }
    }
    sum
}

fn filter_for_loop(n: u64) -> u64 {
    (0..n).filter(|&x| x % 2 == 0).sum()
}

// ==== Zip ====

fn zip_without_sugar(n: u64) -> u64 {
    let v: Vec<u64> = (0..n).collect();
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        let a = i;
        let b = v[i as usize];
        sum += a + b;
        i += 1;
    }
    sum
}

fn zip_for_loop(n: u64) -> u64 {
    let v: Vec<u64> = (0..n).collect();
    (0..n).zip(v.iter()).map(|(a, b)| a + *b).sum()
}

// ==== Skip ====

fn skip_without_sugar(n: u64) -> u64 {
    let mut sum = 0;
    let mut i = 0;
    while i < n {
        if i >= 10 {
            sum += i;
        }
        i += 1;
    }
    sum
}

fn skip_for_loop(n: u64) -> u64 {
    (0..n).skip(10).sum()
}

// ==== Take ====

fn take_without_sugar(n: u64) -> u64 {
    let mut sum = 0;
    let mut count = 0;
    let mut i = 0;
    while i < n && count < 1000 {
        sum += i;
        i += 1;
        count += 1;
    }
    sum
}

fn take_for_loop(n: u64) -> u64 {
    (0..n).take(1000).sum()
}

// ==== Flat Map ====

fn flat_map_without_sugar(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += i;
        sum += i + 1;
    }
    sum
}

fn flat_map_for_loop(n: u64) -> u64 {
    (0..n).flat_map(|x| [x, x + 1].into_iter()).sum()
}

// ==== Any ====

fn any_without_sugar(n: u64) -> bool {
    let mut i = 0;
    while i < n {
        if i == 100 {
            return true;
        }
        i += 1;
    }
    false
}

fn any_for_loop(n: u64) -> bool {
    (0..n).any(|x| x == 100)
}

// ==== All ====

fn all_without_sugar(n: u64) -> bool {
    let mut i = 0;
    while i < n {
        if i >= n {
            return false;
        }
        i += 1;
    }
    true
}

fn all_for_loop(n: u64) -> bool {
    (0..n).all(|x| x < n)
}

// ==== Benchmarks ====

fn bench_for_loops(c: &mut Criterion) {
    let n = 1_000_000;

    c.bench_function("basic_for_loop", |b| {
        b.iter(|| black_box(basic_for_loop(n)))
    });

    c.bench_function("map_without_sugar", |b| {
        b.iter(|| black_box(map_without_sugar(n)))
    });
    c.bench_function("map_for_loop", |b| b.iter(|| black_box(map_for_loop(n))));

    c.bench_function("fold_without_sugar", |b| {
        b.iter(|| black_box(fold_without_sugar(n)))
    });
    c.bench_function("fold_for_loop", |b| b.iter(|| black_box(fold_for_loop(n))));

    c.bench_function("iterator_map_collect_without_sugar", |b| {
        b.iter(|| black_box(iterator_map_collect_without_sugar(n)))
    });
    c.bench_function("iterator_map_collect", |b| {
        b.iter(|| black_box(iterator_map_collect(n)))
    });

    c.bench_function("for_each_without_sugar", |b| {
        b.iter(|| black_box(for_each_without_sugar(n)))
    });
    c.bench_function("for_each_for_loop", |b| {
        b.iter(|| black_box(for_each_for_loop(n)))
    });

    c.bench_function("filter_without_sugar", |b| {
        b.iter(|| black_box(filter_without_sugar(n)))
    });
    c.bench_function("filter_for_loop", |b| {
        b.iter(|| black_box(filter_for_loop(n)))
    });

    c.bench_function("zip_without_sugar", |b| {
        b.iter(|| black_box(zip_without_sugar(n)))
    });
    c.bench_function("zip_for_loop", |b| b.iter(|| black_box(zip_for_loop(n))));

    c.bench_function("skip_without_sugar", |b| {
        b.iter(|| black_box(skip_without_sugar(n)))
    });
    c.bench_function("skip_for_loop", |b| b.iter(|| black_box(skip_for_loop(n))));

    c.bench_function("take_without_sugar", |b| {
        b.iter(|| black_box(take_without_sugar(n)))
    });
    c.bench_function("take_for_loop", |b| b.iter(|| black_box(take_for_loop(n))));

    c.bench_function("flat_map_without_sugar", |b| {
        b.iter(|| black_box(flat_map_without_sugar(n)))
    });
    c.bench_function("flat_map_for_loop", |b| {
        b.iter(|| black_box(flat_map_for_loop(n)))
    });

    c.bench_function("any_without_sugar", |b| {
        b.iter(|| black_box(any_without_sugar(n)))
    });
    c.bench_function("any_for_loop", |b| b.iter(|| black_box(any_for_loop(n))));

    c.bench_function("all_without_sugar", |b| {
        b.iter(|| black_box(all_without_sugar(n)))
    });
    c.bench_function("all_for_loop", |b| b.iter(|| black_box(all_for_loop(n))));
}

criterion_group!(benches, bench_for_loops);
criterion_main!(benches);
