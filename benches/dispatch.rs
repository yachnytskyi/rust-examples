use criterion::{Criterion, criterion_group, criterion_main};
use std::collections::HashMap;

// --- Large struct by value ---
#[derive(Clone)]
struct UserProfile {
    id: i32,
    name: String,
    email: String,
    history: [u64; 1024],
}

trait RepoTrait {
    fn find_by_id(&self, id: i32) -> Option<&UserProfile>;
    fn save(&mut self, profile: UserProfile);
}

// --- InMemoryRepo implements the trait ---
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
    fn find_by_id_pure(&self, id: i32) -> Option<&UserProfile> {
        self.data.get(&id)
    }
    fn save_pure(&mut self, profile: UserProfile) {
        self.data.insert(profile.id, profile);
    }
}

impl RepoTrait for InMemoryRepo {
    fn find_by_id(&self, id: i32) -> Option<&UserProfile> {
        self.data.get(&id)
    }
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
    fn sum_users(&mut self, n: usize) -> u64 {
        for i in 0..n {
            let profile = UserProfile {
                id: i as i32,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                history: [i as u64; 1024],
            };
            self.repo.save_pure(profile);
        }
        let mut sum = 0u64;
        for i in 0..n {
            if let Some(p) = self.repo.find_by_id_pure(i as i32) {
                sum += p.history[0];
            }
        }
        sum
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
    fn sum_users(&mut self, n: usize) -> u64 {
        for i in 0..n {
            let profile = UserProfile {
                id: i as i32,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                history: [i as u64; 1024],
            };
            self.repo.save(profile);
        }
        let mut sum = 0u64;
        for i in 0..n {
            if let Some(p) = self.repo.find_by_id(i as i32) {
                sum += p.history[0];
            }
        }
        sum
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
    fn sum_users(&mut self, n: usize) -> u64 {
        for i in 0..n {
            let profile = UserProfile {
                id: i as i32,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                history: [i as u64; 1024],
            };
            self.repo.save(profile);
        }
        let mut sum = 0u64;
        for i in 0..n {
            if let Some(p) = self.repo.find_by_id(i as i32) {
                sum += p.history[0];
            }
        }
        sum
    }
}

fn bench_usecase_dispatch(c: &mut Criterion) {
    let n = 100;

    c.bench_function("usecase_pure_static", |b| {
        b.iter(|| {
            let repo = InMemoryRepo::new();
            let mut usecase = UserUseCasePure::new(repo);
            usecase.sum_users(n)
        })
    });

    c.bench_function("usecase_static_dispatch", |b| {
        b.iter(|| {
            let repo = InMemoryRepo::new();
            let mut usecase = UserUseCaseStatic::new(repo);
            usecase.sum_users(n)
        })
    });

    c.bench_function("usecase_dynamic_dispatch", |b| {
        b.iter(|| {
            let mut repo = InMemoryRepo::new();
            let mut usecase = UserUseCaseDynamic::new(&mut repo as &mut dyn RepoTrait);
            usecase.sum_users(n)
        })
    });
}

// --- Error types ---

#[derive(Debug)]
enum StaticError {
    NotFound,
}
type StaticResult<T> = Result<T, StaticError>;

use std::error::Error;

#[derive(Debug)]
struct NotFound;
impl std::fmt::Display for NotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "not found")
    }
}
impl Error for NotFound {}
type DynamicResult<T> = Result<T, Box<dyn Error>>;

// --- Error functions ---

fn static_error_example(i: usize) -> StaticResult<usize> {
    if i % 2 == 0 {
        Ok(i)
    } else {
        Err(StaticError::NotFound)
    }
}

fn dynamic_error_example(i: usize) -> DynamicResult<usize> {
    if i % 2 == 0 {
        Ok(i)
    } else {
        Err(Box::new(NotFound))
    }
}

// --- Benchmarks ---

fn bench_error_handling(c: &mut Criterion) {
    let n = 10_000;

    c.bench_function("error_static_enum", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..n {
                if let Ok(x) = static_error_example(i) {
                    sum += x;
                }
            }
            sum
        })
    });

    c.bench_function("error_dynamic_boxed", |b| {
        b.iter(|| {
            let mut sum = 0;
            for i in 0..n {
                if let Ok(x) = dynamic_error_example(i) {
                    sum += x;
                }
            }
            sum
        })
    });
}

criterion_group!(benches, bench_usecase_dispatch, bench_error_handling);
criterion_main!(benches);

