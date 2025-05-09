use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;

// Define the structures
#[derive(Clone)]
struct Small {
    a: i32,
    b: String,
    c: f64,
    d: Vec<u8>,
    e: HashMap<String, i32>,
    f: bool,
    g: Option<i32>,
    h: Vec<String>,
    i: f32,
    j: HashMap<String, Vec<i32>>,
}

#[derive(Clone)]
struct Medium {
    a: i32,
    b: String,
    c: f64,
    d: Vec<u8>,
    e: HashMap<String, i32>,
    f: i64,
    g: Vec<i32>,
    h: String,
    i: HashMap<i32, String>,
    j: f32,
    k: Option<i32>,
    l: Vec<f64>,
    m: bool,
    n: Vec<String>,
    o: HashMap<i64, f64>,
    p: String,
    q: Option<Vec<i32>>,
    r: Vec<u32>,
    s: HashMap<String, Vec<u8>>,
    t: f64,
}

#[derive(Clone)]
struct Large {
    a: i64,
    b: f32,
    c: String,
    d: Vec<u32>,
    e: HashMap<String, Vec<u8>>,
    f: Option<u64>,
    g: i64,
    h: Vec<i32>,
    i: HashMap<i32, String>,
    j: f64,
    k: u32,
    l: Vec<f64>,
    m: String,
    n: HashMap<i64, i32>,
    o: Vec<String>,
    p: Vec<i64>,
    q: Vec<u8>,
    r: HashMap<String, String>,
    s: Vec<u32>,
    t: Option<Vec<i32>>,
    u: f64,
    v: String,
    w: i32,
    x: f32,
    y: i64,
    z: String,
    aa: Vec<f32>,
    ab: u64,
    ac: HashMap<String, Vec<i32>>,
    ad: Option<String>,
    ae: Vec<String>,
    af: HashMap<i64, f64>,
}

// The repository that returns either owned structures or references
struct Repository {
    small: Small,
    medium: Medium,
    large: Large,
}

impl Repository {
    fn new() -> Self {
        Repository {
            small: Small {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: vec!["Apple".to_string(), "Banana".to_string()],
                i: 2.71,
                j: HashMap::new(),
            },
            medium: Medium {
                a: 2,
                b: "World".to_string(),
                c: 3.14,
                d: vec![1, 2, 3, 4],
                e: HashMap::new(),
                f: 123,
                g: vec![1, 2, 3],
                h: "Medium struct".to_string(),
                i: HashMap::new(),
                j: 2.71,
                k: Some(42),
                l: vec![1.0, 2.0, 3.0],
                m: true,
                n: vec!["Apple".to_string(), "Banana".to_string()],
                o: HashMap::new(),
                p: "String".to_string(),
                q: Some(vec![4, 5, 6]),
                r: vec![10, 20, 30],
                s: HashMap::new(),
                t: 3.14159,
            },
            large: Large {
                a: 1,
                b: 3.14,
                c: "Large struct".to_string(),
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: Some(42),
                g: 10,
                h: vec![1, 2, 3],
                i: HashMap::new(),
                j: 2.718,
                k: 5,
                l: vec![1.0, 2.0, 3.0],
                m: "Hello".to_string(),
                n: HashMap::new(),
                o: vec!["Apple".to_string(), "Banana".to_string()],
                p: vec![1, 2, 3],
                q: vec![1, 2, 3],
                r: HashMap::new(),
                s: vec![1, 2, 3],
                t: Some(vec![4, 5, 6]),
                u: 1.618,
                v: "Test".to_string(),
                w: 9,
                x: 6.28,
                y: 1000,
                z: "End".to_string(),
                aa: vec![1.0, 2.0, 3.0],
                ab: 100,
                ac: HashMap::new(),
                ad: Some("End of struct".to_string()),
                ae: vec!["Hello".to_string(), "Rust".to_string()],
                af: HashMap::new(),
            },
        }
    }

    // Return references to structures
    fn get_ref_small(&self) -> &Small {
        &self.small
    }

    fn get_ref_medium(&self) -> &Medium {
        &self.medium
    }

    fn get_ref_large(&self) -> &Large {
        &self.large
    }

    // Return mutable references to structures
    fn get_mut_ref_small(&mut self) -> &mut Small {
        &mut self.small
    }

    fn get_mut_ref_medium(&mut self) -> &mut Medium {
        &mut self.medium
    }

    fn get_mut_ref_large(&mut self) -> &mut Large {
        &mut self.large
    }
}

// The Use Case layer that fetches data from the Repository
struct UseCase<'a> {
    repository: &'a mut Repository,
}

impl<'a> UseCase<'a> {
    fn new(repository: &'a mut Repository) -> Self {
        UseCase { repository }
    }

    fn get_ref_small(&self) -> &Small {
        self.repository.get_ref_small()
    }

    fn get_ref_medium(&self) -> &Medium {
        self.repository.get_ref_medium()
    }

    fn get_ref_large(&self) -> &Large {
        self.repository.get_ref_large()
    }

    fn get_mut_ref_small(&mut self) -> &mut Small {
        self.repository.get_mut_ref_small()
    }

    fn get_mut_ref_medium(&mut self) -> &mut Medium {
        self.repository.get_mut_ref_medium()
    }

    fn get_mut_ref_large(&mut self) -> &mut Large {
        self.repository.get_mut_ref_large()
    }
}

// The Controller layer that interacts with the Use Case layer
struct Controller<'a> {
    use_case: &'a mut UseCase<'a>,
}

impl<'a> Controller<'a> {
    fn new(use_case: &'a mut UseCase<'a>) -> Self {
        Controller { use_case }
    }

    fn get_ref_small(&self) -> &Small {
        self.use_case.get_ref_small()
    }

    fn get_ref_medium(&self) -> &Medium {
        self.use_case.get_ref_medium()
    }

    fn get_ref_large(&self) -> &Large {
        self.use_case.get_ref_large()
    }

    fn get_mut_ref_small(&mut self) -> &mut Small {
        self.use_case.get_mut_ref_small()
    }

    fn get_mut_ref_medium(&mut self) -> &mut Medium {
        self.use_case.get_mut_ref_medium()
    }

    fn get_mut_ref_large(&mut self) -> &mut Large {
        self.use_case.get_mut_ref_large()
    }
}

// Benchmark function for the three-layer approach: Controller -> Use Case -> Repository
fn benchmark_three_layer_ref_small(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_ref_small", |b| {
        b.iter(|| {
            let _ = black_box(controller.get_ref_small());
        })
    });
}

fn benchmark_three_layer_ref_medium(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_ref_medium", |b| {
        b.iter(|| {
            let _ = black_box(controller.get_ref_medium());
        })
    });
}

fn benchmark_three_layer_ref_large(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_ref_large", |b| {
        b.iter(|| {
            let _ = black_box(controller.get_ref_large());
        })
    });
}

fn benchmark_three_layer_mut_ref_small(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_mut_ref_small", |b| {
        b.iter(|| {
            let _ = black_box(controller.get_mut_ref_small());
        })
    });
}

fn benchmark_three_layer_mut_ref_medium(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_mut_ref_medium", |b| {
        b.iter(|| {
            let _ = black_box(controller.get_mut_ref_medium());
        })
    });
}

fn benchmark_three_layer_mut_ref_large(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_mut_ref_large", |b| {
        b.iter(|| {
            let _ = black_box(controller.get_mut_ref_large());
        })
    });
}

// New Move Benchmark Functions

fn benchmark_three_layer_move_small(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_move_small", |b| {
        b.iter(|| {
            let moved_data = controller.get_ref_small().clone(); // Move the owned data
            black_box(moved_data);
        })
    });
}

fn benchmark_three_layer_move_medium(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_move_medium", |b| {
        b.iter(|| {
            let moved_data = controller.get_ref_medium().clone(); // Move the owned data
            black_box(moved_data);
        })
    });
}

fn benchmark_three_layer_move_large(c: &mut Criterion) {
    let mut repository = Repository::new();
    let mut use_case = UseCase::new(&mut repository);
    let mut controller = Controller::new(&mut use_case);

    c.bench_function("controller_to_use_case_to_repo_move_large", |b| {
        b.iter(|| {
            let moved_data = controller.get_ref_large().clone(); // Move the owned data
            black_box(moved_data);
        })
    });
}

criterion_group!(
    benches,
    benchmark_three_layer_ref_small,
    benchmark_three_layer_ref_medium,
    benchmark_three_layer_ref_large,
    benchmark_three_layer_mut_ref_small,
    benchmark_three_layer_mut_ref_medium,
    benchmark_three_layer_mut_ref_large,
    benchmark_three_layer_move_small,
    benchmark_three_layer_move_medium,    
    benchmark_three_layer_move_large 
);

criterion_main!(benches);
