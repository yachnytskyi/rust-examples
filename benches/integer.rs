use criterion::{Criterion, black_box, criterion_group, criterion_main};

//
// i64 versions
//

fn takes_i64_by_move(value: i64) -> i64 {
    black_box(value + 1)
}

fn takes_i64_by_ref(value: &i64) -> i64 {
    black_box(value + 1) * value
}

fn takes_and_returns_i64_by_ref(value: &i64) -> &i64 {
    black_box(value + 1);
    value
}

fn takes_i64_by_move_without_return(value: i64) {
    black_box(value + 1);
}

fn takes_without_returning_i64_by_ref(value: &i64) {
    black_box(value + 1);
}

fn benchmark_i64_functions(c: &mut Criterion) {
    let value: i64 = 1234567890123456789;

    c.bench_function("i64::takes_by_move", |b| {
        b.iter(|| {
            let result = takes_i64_by_move(black_box(value));
            black_box(result);
        })
    });

    c.bench_function("i64::takes_by_ref", |b| {
        b.iter(|| {
            let result = takes_i64_by_ref(black_box(&value));
            black_box(result);
        })
    });

    c.bench_function("i64::takes_and_returns_by_ref", |b| {
        b.iter(|| {
            let result = takes_and_returns_i64_by_ref(black_box(&value));
            black_box(result);
        })
    });

    c.bench_function("i64::takes_by_move_without_return", |b| {
        b.iter(|| {
            let result = takes_i64_by_move_without_return(black_box(value));
            black_box(result);
        })
    });

    c.bench_function("i64::takes_without_returning_by_ref", |b| {
        b.iter(|| {
            takes_without_returning_i64_by_ref(black_box(&value));
        })
    });
}

//
// i128 versions
//

fn takes_i128_by_move(value: i128) -> i128 {
    black_box(value + 1)
}

fn takes_i128_by_ref(value: &i128) -> i128 {
    black_box(value + 1) * value
}

fn takes_and_returns_i128_by_ref(value: &i128) -> &i128 {
    black_box(value + 1);
    value
}

fn takes_i128_by_move_without_return(value: i128) {
    black_box(value + 1);
}

fn takes_without_returning_i128_by_ref(value: &i128) {
    black_box(value + 1);
}

fn benchmark_i128_functions(c: &mut Criterion) {
    let value: i128 = 123456789012345678901234567890;

    c.bench_function("i128::takes_by_move", |b| {
        b.iter(|| {
            let result = takes_i128_by_move(black_box(value));
            black_box(result);
        })
    });

    c.bench_function("i128::takes_by_ref", |b| {
        b.iter(|| {
            let result = takes_i128_by_ref(black_box(&value));
            black_box(result);
        })
    });

    c.bench_function("i128::takes_and_returns_by_ref", |b| {
        b.iter(|| {
            let result = takes_and_returns_i128_by_ref(black_box(&value));
            black_box(result);
        })
    });

    c.bench_function("i128::takes_by_move_without_return", |b| {
        b.iter(|| {
            let result = takes_i128_by_move_without_return(black_box(value));
            black_box(result);
        })
    });

    c.bench_function("i128::takes_without_returning_by_ref", |b| {
        b.iter(|| {
            takes_without_returning_i128_by_ref(black_box(&value));
        })
    });
}

criterion_group!(benches, benchmark_i64_functions, benchmark_i128_functions);
criterion_main!(benches);
