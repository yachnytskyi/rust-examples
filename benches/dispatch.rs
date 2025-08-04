use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;
use std::error::Error;

// --- Data model (reduce large-by-value cost by boxing the array) ---
struct UserProfile {
    id: i32,
    name: &'static str,
    email: &'static str,
    history: Box<[u64; 1024]>,
}

impl UserProfile {
    #[inline(never)]
    fn new(i: usize) -> Self {
        Self {
            id: i as i32,
            name: "Alice",
            email: "alice@example.com",
            history: Box::new([i as u64; 1024]),
        }
    }
}

// --- Repository trait and impls ---

trait RepoTrait {
    fn find_by_id(&self, id: i32) -> Option<&UserProfile>;
    fn save(&mut self, profile: UserProfile);
}

// InMemoryRepo implements both trait methods and "pure" methods (non-trait)
struct InMemoryRepo {
    data: HashMap<i32, UserProfile>,
}

impl InMemoryRepo {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    // Pure static methods (no trait at all)
    #[inline(never)]
    fn find_by_id_pure(&self, id: i32) -> Option<&UserProfile> {
        self.data.get(&id)
    }
    #[inline(never)]
    fn save_pure(&mut self, profile: UserProfile) {
        self.data.insert(profile.id, profile);
    }
}

impl RepoTrait for InMemoryRepo {
    #[inline(never)]
    fn find_by_id(&self, id: i32) -> Option<&UserProfile> {
        self.data.get(&id)
    }
    #[inline(never)]
    fn save(&mut self, profile: UserProfile) {
        self.data.insert(profile.id, profile);
    }
}

// --- Use case "service" layer ---

// (1) Pure static use case
struct UserUseCasePure {
    repo: InMemoryRepo,
}
impl UserUseCasePure {
    fn new(repo: InMemoryRepo) -> Self {
        Self { repo }
    }
    #[inline(never)]
    fn sum_users(&mut self, n: usize) -> u64 {
        let n = black_box(n);
        for i in 0..n {
            let profile = UserProfile::new(black_box(i));
            self.repo.save_pure(profile);
        }
        let mut sum = 0u64;
        for i in 0..n {
            if let Some(p) = self.repo.find_by_id_pure(i as i32) {
                // black_box to keep the read live
                sum = sum.wrapping_add(black_box(p.history[0]));
            }
        }
        black_box(sum)
    }
}

// (2) Static dispatch (generic trait bound)
struct UserUseCaseStatic<R: RepoTrait> {
    repo: R,
}
impl<R: RepoTrait> UserUseCaseStatic<R> {
    fn new(repo: R) -> Self {
        Self { repo }
    }
    #[inline(never)]
    fn sum_users(&mut self, n: usize) -> u64 {
        let n = black_box(n);
        for i in 0..n {
            let profile = UserProfile::new(black_box(i));
            self.repo.save(profile);
        }
        let mut sum = 0u64;
        for i in 0..n {
            if let Some(p) = self.repo.find_by_id(i as i32) {
                sum = sum.wrapping_add(black_box(p.history[0]));
            }
        }
        black_box(sum)
    }
}

// (3) Dynamic dispatch use case
struct UserUseCaseDynamic<'a> {
    repo: &'a mut dyn RepoTrait,
}
impl<'a> UserUseCaseDynamic<'a> {
    fn new(repo: &'a mut dyn RepoTrait) -> Self {
        Self { repo }
    }
    #[inline(never)]
    fn sum_users(&mut self, n: usize) -> u64 {
        let n = black_box(n);
        for i in 0..n {
            let profile = UserProfile::new(black_box(i));
            self.repo.save(profile);
        }
        let mut sum = 0u64;
        for i in 0..n {
            if let Some(p) = self.repo.find_by_id(i as i32) {
                sum = sum.wrapping_add(black_box(p.history[0]));
            }
        }
        black_box(sum)
    }
}

// --- Bench: usecase dispatch (note: still dominated by HashMap/allocs) ---

fn bench_usecase_dispatch(c: &mut Criterion) {
    let n = 1_000; // keep modest; this bench is illustrative

    c.bench_function("usecase_pure_methods", |b| {
        b.iter(|| {
            let repo = InMemoryRepo::new();
            let mut usecase = UserUseCasePure::new(repo);
            black_box(usecase.sum_users(black_box(n)))
        })
    });

    c.bench_function("usecase_static_dispatch", |b| {
        b.iter(|| {
            let repo = InMemoryRepo::new();
            let mut usecase = UserUseCaseStatic::new(repo);
            black_box(usecase.sum_users(black_box(n)))
        })
    });

    c.bench_function("usecase_dynamic_dispatch", |b| {
        b.iter(|| {
            let mut repo = InMemoryRepo::new();
            let mut usecase = UserUseCaseDynamic::new(&mut repo as &mut dyn RepoTrait);
            black_box(usecase.sum_users(black_box(n)))
        })
    });
}

// --- Minimal dispatch-only microbench (to actually see dispatch overhead) ---

trait Counter {
    fn add(&mut self, x: u64);
    fn get(&self) -> u64;
}

struct PlainCounter(u64);

impl Counter for PlainCounter {
    #[inline(never)]
    fn add(&mut self, x: u64) {
        self.0 = self.0.wrapping_add(x);
    }
    #[inline(never)]
    fn get(&self) -> u64 {
        self.0
    }
}

struct UseStatic<C: Counter>(C);
impl<C: Counter> UseStatic<C> {
    #[inline(never)]
    fn run(&mut self, n: usize) -> u64 {
        let n = black_box(n);
        for i in 0..n {
            self.0.add(black_box(i as u64));
        }
        black_box(self.0.get())
    }
}

struct UseDynamic<'a> {
    c: &'a mut dyn Counter,
}
impl<'a> UseDynamic<'a> {
    #[inline(never)]
    fn run(&mut self, n: usize) -> u64 {
        let n = black_box(n);
        for i in 0..n {
            self.c.add(black_box(i as u64));
        }
        black_box(self.c.get())
    }
}

fn bench_dispatch_minimal(c: &mut Criterion) {
    let n = 5_000_000;

    c.bench_function("dispatch_static_minimal", |b| {
        b.iter(|| {
            let mut uc = UseStatic(PlainCounter(0));
            black_box(uc.run(black_box(n)))
        })
    });

    c.bench_function("dispatch_dynamic_minimal", |b| {
        b.iter(|| {
            let mut pc = PlainCounter(0);
            let mut uc = UseDynamic {
                c: &mut pc as &mut dyn Counter,
            };
            black_box(uc.run(black_box(n)))
        })
    });
}

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

#[inline(never)]
fn static_error_example(i: usize) -> StaticResult<usize> {
    let i = black_box(i);
    if i % 2 == 0 {
        Ok(i)
    } else {
        Err(StaticError::NotFound)
    }
}

#[inline(never)]
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

criterion_group!(
    benches,
    bench_usecase_dispatch,
    bench_dispatch_minimal,
    bench_error_handling
);
criterion_main!(benches);
