use criterion::{Criterion, black_box, criterion_group, criterion_main};

// Function that takes i128 by move (ownership, but i128 is Copy)
fn takes_i128_by_move(value: i128) -> i128 {
    // do some trivial operation to prevent optimization
    black_box(value + 1)
}

// Function that takes i128 by reference (borrow)
fn takes_i128_by_ref(value: &i128) -> i128 {
    black_box(value + 1) * value
}

// Function that takes i128 by reference (borrow)
fn takes_and_returns_i128_by_ref(value: &i128) -> &i128 {
    black_box(value + 1);
    value
}

// Function that takes i128 by reference (borrow)
fn takes_without_returning_i128_by_ref(value: &i128) {
    black_box(value + 1);
}

fn benchmark_i128_functions(c: &mut Criterion) {
    let value: i128 = 123456789012345678901234567890;

    c.bench_function("takes_i128_by_move", |b| {
        b.iter(|| {
            let result = takes_i128_by_move(black_box(value));
            black_box(result);
        })
    });

    c.bench_function("takes_i128_by_ref", |b| {
        b.iter(|| {
            let result = takes_i128_by_ref(black_box(&value));
            black_box(result);
        })
    });

    c.bench_function("takes_and_returns_i128_by_ref", |b| {
        b.iter(|| {
            let result = takes_and_returns_i128_by_ref(black_box(&value));
            black_box(result);
        })
    });

    c.bench_function("takes_without_returning_i128_by_ref", |b| {
        b.iter(|| {
            let result = takes_without_returning_i128_by_ref(black_box(&value));
            black_box(result);
        })
    });
}

criterion_group!(benches, benchmark_i128_functions);
criterion_main!(benches);
