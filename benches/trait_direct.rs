use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::error::Error;
use std::fmt;

// ==== Original dispatch benchmarks ====

trait UserRepo {
    fn get_user(&self, id: u64) -> u64;
}

struct ConcreteRepo;

impl UserRepo for ConcreteRepo {
    #[inline(never)]
    fn get_user(&self, _id: u64) -> u64 {
        42
    }
}

#[inline(never)]
fn use_case_concrete(repo: &ConcreteRepo, id: u64) -> u64 {
    repo.get_user(id)
}

#[inline(never)]
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

// ==== New extended benchmarks ====

trait Worker {
    fn work(&self) -> u64;
}

struct Concrete;

impl Worker for Concrete {
    #[inline(never)]
    fn work(&self) -> u64 {
        42
    }
}

#[inline(never)]
fn concrete_call(worker: &Concrete, n: u64) -> u64 {
    let mut sum = 0;
    for _ in 0..n {
        sum += worker.work();
    }
    sum
}

#[inline(never)]
fn trait_object_call(worker: &dyn Worker, n: u64) -> u64 {
    let mut sum = 0;
    for _ in 0..n {
        sum += worker.work();
    }
    sum
}

#[inline(never)]
fn boxed_trait_object_call(worker: Box<dyn Worker>, n: u64) -> u64 {
    let mut sum = 0;
    for _ in 0..n {
        sum += worker.work();
    }
    sum
}

#[inline(never)]
fn function_pointer_call(f: fn() -> u64, n: u64) -> u64 {
    let mut sum = 0;
    for _ in 0..n {
        sum += f();
    }
    sum
}

#[inline(never)]
fn generic_call<T: Worker>(worker: &T, n: u64) -> u64 {
    let mut sum = 0;
    for _ in 0..n {
        sum += worker.work();
    }
    sum
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

#[inline(never)]
fn concrete_error_fn(success: bool) -> Result<u64, MyError> {
    if success {
        Ok(42)
    } else {
        Err(MyError)
    }
}

#[inline(never)]
fn dynamic_error_fn(success: bool) -> Result<u64, Box<dyn Error>> {
    if success {
        Ok(42)
    } else {
        Err(Box::new(MyError))
    }
}

fn bench_error_dispatch(c: &mut Criterion) {
    let n = 1_000_000;

    c.bench_function("concrete_error_result", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..n {
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
            for i in 0..n {
                match black_box(dynamic_error_fn(true)) {
                    Ok(v) => sum += v,
                    Err(_) => {}
                }
            }
            black_box(sum)
        })
    });
}

// ==== Benchmark groups ====

criterion_group!(
    benches,
    bench_dispatch_overhead,
    bench_more_dispatch_types,
    bench_error_dispatch
);
criterion_main!(benches);

fn bench_more_dispatch_types(c: &mut Criterion) {
    let worker = Concrete;

    let n = 1_000_000;

    c.bench_function("new_concrete_call", |b| {
        b.iter(|| black_box(concrete_call(&worker, n)))
    });

    c.bench_function("new_trait_object_call", |b| {
        b.iter(|| black_box(trait_object_call(&worker, n)))
    });

    c.bench_function("new_boxed_trait_object_call", |b| {
        b.iter(|| black_box(boxed_trait_object_call(Box::new(Concrete), n)))
    });

    c.bench_function("new_function_pointer_call", |b| {
        b.iter(|| black_box(function_pointer_call(|| 42, n)))
    });

    c.bench_function("new_generic_call", |b| {
        b.iter(|| black_box(generic_call(&worker, n)))
    });
}
