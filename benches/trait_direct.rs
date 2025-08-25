use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::error::Error;
use std::fmt;

// ==== Original dispatch benchmarks ====

trait UserRepo {
    fn get_user(&self, id: u64) -> u64;
}

struct ConcreteRepo;

impl UserRepo for ConcreteRepo {
    fn get_user(&self, _id: u64) -> u64 {
        42
    }
}

fn use_case_concrete(repo: &ConcreteRepo, id: u64) -> u64 {
    repo.get_user(id)
}

fn use_case_trait(repo: &dyn UserRepo, id: u64) -> u64 {
    repo.get_user(id)
}

fn bench_dispatch_overhead(c: &mut Criterion) {
    let repo = ConcreteRepo;
    let repo_trait: &dyn UserRepo = &repo;

    c.bench_function("original_concrete_call", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..1_000_000 {
                sum += black_box(use_case_concrete(&repo, i));
            }
            black_box(sum)
        })
    });

    c.bench_function("original_trait_object_call", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..1_000_000 {
                sum += black_box(use_case_trait(repo_trait, i));
            }
            black_box(sum)
        })
    });
}

// ==== Error benchmarks ====

#[derive(Debug)]
struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyError")
    }
}

impl Error for MyError {}

fn concrete_error_fn(success: bool) -> Result<u64, MyError> {
    if success { Ok(42) } else { Err(MyError) }
}

fn dynamic_error_fn(success: bool) -> Result<u64, Box<dyn Error>> {
    if success {
        Ok(42)
    } else {
        Err(Box::new(MyError))
    }
}

// ==== Enum error type and benchmark ====

#[derive(Debug)]
enum MyEnumError {
    Variant,
}

impl fmt::Display for MyEnumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MyEnumError::Variant")
    }
}

impl Error for MyEnumError {}

fn enum_error_fn(success: bool) -> Result<u64, MyEnumError> {
    if success {
        Ok(42)
    } else {
        Err(MyEnumError::Variant)
    }
}

fn bench_error_dispatch(c: &mut Criterion) {
    let n = 1_000_000;

    c.bench_function("concrete_error_result", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..n {
                match black_box(concrete_error_fn(true)) {
                    Ok(v) => sum += v,
                    Err(_) => {}
                }
            }
            black_box(sum)
        })
    });

    c.bench_function("dynamic_error_result", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..n {
                match black_box(dynamic_error_fn(true)) {
                    Ok(v) => sum += v,
                    Err(_) => {}
                }
            }
            black_box(sum)
        })
    });

    c.bench_function("enum_error_result", |b| {
        b.iter(|| {
            let mut sum = 0;
            for _ in 0..n {
                match black_box(enum_error_fn(true)) {
                    Ok(v) => sum += v,
                    Err(_) => {}
                }
            }
            black_box(sum)
        })
    });
}

// ==== Benchmark groups ====

criterion_group!(benches, bench_dispatch_overhead, bench_error_dispatch);
criterion_main!(benches);
