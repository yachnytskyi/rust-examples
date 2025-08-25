use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;
use std::error::Error;

// --- Error types (make boxed error non-trivial so it actually allocates) ---

#[derive(Debug)]
enum StaticError {
    NotFound,
}
type StaticResult<T> = Result<T, StaticError>;

#[derive(Debug)]
struct NotFound(String);
impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for NotFound {}
type DynamicResult<T> = Result<T, Box<dyn Error>>;

// --- Error functions (prevent inlining; use black_box) ---

fn static_error_example(i: usize) -> StaticResult<usize> {
    let i = black_box(i);
    if i % 2 == 0 {
        Ok(i)
    } else {
        Err(StaticError::NotFound)
    }
}

fn dynamic_error_example(i: usize) -> DynamicResult<usize> {
    let i = black_box(i);
    if i % 2 == 0 {
        Ok(i)
    } else {
        // Force real work on the error path: allocate a String and a Box<dyn Error>
        Err(Box::new(NotFound(format!("{} not found", i))))
    }
}

// --- Bench: error handling ---

fn bench_error_handling(c: &mut Criterion) {
    let n = 50_000;

    c.bench_function("error_static_enum", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for i in 0..n {
                match black_box(static_error_example(i)) {
                    Ok(x) => sum = sum.wrapping_add(black_box(x)),
                    Err(_) => {}
                }
            }
            black_box(sum)
        })
    });

    c.bench_function("error_dynamic_boxed", |b| {
        b.iter(|| {
            let mut sum = 0usize;
            for i in 0..n {
                match black_box(dynamic_error_example(i)) {
                    Ok(x) => sum = sum.wrapping_add(black_box(x)),
                    Err(_) => {}
                }
            }
            black_box(sum)
        })
    });
}

criterion_group!(benches, bench_error_handling);
criterion_main!(benches);
