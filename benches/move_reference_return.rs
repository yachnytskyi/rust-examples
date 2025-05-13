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

// Repository Layer: Handles data access and returns structs (Boxed or Owned)
struct Repository;

impl Repository {
    // Returns a moved Small struct
    fn get_small(&self) -> Small {
        Small {
            a: 1,
            b: "text".into(),
            c: 3.14,
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: true,
            g: Some(42),
            h: vec!["hello".into()],
            i: 2.5,
            j: HashMap::new(),
        }
    }

    // Returns a moved Medium struct
    fn get_medium(&self) -> Medium {
        Medium {
            a: 1,
            b: "text".into(),
            c: 3.14,
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: 2,
            g: vec![1, 2, 3],
            h: "medium".into(),
            i: HashMap::new(),
            j: 3.14,
            k: Some(42),
            l: vec![1.1, 2.2],
            m: true,
            n: vec!["one".into(), "two".into()],
            o: HashMap::new(),
            p: "p".into(),
            q: Some(vec![1, 2]),
            r: vec![1, 2],
            s: HashMap::new(),
            t: 4.0,
        }
    }

    // Returns a moved Large struct
    fn get_large(&self) -> Large {
        Large {
            a: 1,
            b: 2.0,
            c: "Large struct".into(),
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: Some(4),
            g: 5,
            h: vec![1, 2, 3],
            i: HashMap::new(),
            j: 6.7,
            k: 8,
            l: vec![1.1, 2.2],
            m: "Hello".into(),
            n: HashMap::new(),
            o: vec!["a".into()],
            p: vec![1, 2],
            q: vec![10, 11],
            r: HashMap::new(),
            s: vec![10, 20],
            t: Some(vec![1, 2]),
            u: 10.0,
            v: "test".into(),
            w: 100,
            x: 10.0,
            y: 1000,
            z: "world".into(),
            aa: vec![1.0, 2.0],
            ab: 1000,
            ac: HashMap::new(),
            ad: Some("string".into()),
            ae: vec!["test".into()],
            af: HashMap::new(),
        }
    }

    // Returns a Boxed Small struct
    fn get_small_boxed(&self) -> Box<Small> {
        Box::new(Small {
            a: 1,
            b: "text".into(),
            c: 3.14,
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: true,
            g: Some(42),
            h: vec!["hello".into()],
            i: 2.5,
            j: HashMap::new(),
        })
    }

    // Returns a Boxed Medium struct
    fn get_medium_boxed(&self) -> Box<Medium> {
        Box::new(Medium {
            a: 1,
            b: "text".into(),
            c: 3.14,
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: 2,
            g: vec![1, 2, 3],
            h: "medium".into(),
            i: HashMap::new(),
            j: 3.14,
            k: Some(42),
            l: vec![1.1, 2.2],
            m: true,
            n: vec!["one".into(), "two".into()],
            o: HashMap::new(),
            p: "p".into(),
            q: Some(vec![1, 2]),
            r: vec![1, 2],
            s: HashMap::new(),
            t: 4.0,
        })
    }

    // Returns a Boxed Large struct
    fn get_large_boxed(&self) -> Box<Large> {
        Box::new(Large {
            a: 1,
            b: 2.0,
            c: "Large struct".into(),
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: Some(4),
            g: 5,
            h: vec![1, 2, 3],
            i: HashMap::new(),
            j: 6.7,
            k: 8,
            l: vec![1.1, 2.2],
            m: "Hello".into(),
            n: HashMap::new(),
            o: vec!["a".into()],
            p: vec![1, 2],
            q: vec![10, 11],
            r: HashMap::new(),
            s: vec![10, 20],
            t: Some(vec![1, 2]),
            u: 10.0,
            v: "test".into(),
            w: 100,
            x: 10.0,
            y: 1000,
            z: "world".into(),
            aa: vec![1.0, 2.0],
            ab: 1000,
            ac: HashMap::new(),
            ad: Some("string".into()),
            ae: vec!["test".into()],
            af: HashMap::new(),
        })
    }
}

// UseCase Layer: Accepts references to Repository and processes data
struct UseCase<'a> {
    repo: &'a Repository,
}

impl<'a> UseCase<'a> {
    fn new(repo: &'a Repository) -> Self {
        UseCase { repo }
    }

    // Move-based: Processes and moves struct ownership
    fn process_small(&self) -> Small {
        self.repo.get_small()
    }

    fn process_medium(&self) -> Medium {
        self.repo.get_medium()
    }

    fn process_large(&self) -> Large {
        self.repo.get_large()
    }

    // Box-based: Processes and returns Boxed struct
    fn process_small_boxed(&self) -> Box<Small> {
        self.repo.get_small_boxed()
    }

    fn process_medium_boxed(&self) -> Box<Medium> {
        self.repo.get_medium_boxed()
    }

    fn process_large_boxed(&self) -> Box<Large> {
        self.repo.get_large_boxed()
    }
}

// Controller Layer: Accepts UseCase and coordinates the request
struct Controller<'a> {
    use_case: &'a UseCase<'a>,
}

impl<'a> Controller<'a> {
    fn new(use_case: &'a UseCase<'a>) -> Self {
        Controller { use_case }
    }

    // Move-based: Calls UseCase and moves struct ownership
    fn run_small(&self) -> Small {
        self.use_case.process_small()
    }

    fn run_medium(&self) -> Medium {
        self.use_case.process_medium()
    }

    fn run_large(&self) -> Large {
        self.use_case.process_large()
    }

    // Box-based: Calls UseCase and returns Boxed struct
    fn run_small_boxed(&self) -> Box<Small> {
        self.use_case.process_small_boxed()
    }

    fn run_medium_boxed(&self) -> Box<Medium> {
        self.use_case.process_medium_boxed()
    }

    fn run_large_boxed(&self) -> Box<Large> {
        self.use_case.process_large_boxed()
    }
}

// Benchmark for the 3-layer architecture approach
fn benchmark_3layer_architecture(c: &mut Criterion) {
    let repo = Repository;
    let use_case = UseCase::new(&repo);
    let controller = Controller::new(&use_case);

    // Moving the structure through the layers
    c.bench_function("move_small", |b| {
        b.iter(|| {
            let result = black_box(controller.run_small());
        })
    });

    c.bench_function("move_medium", |b| {
        b.iter(|| {
            let result = black_box(controller.run_medium());
        })
    });

    c.bench_function("move_large", |b| {
        b.iter(|| {
            let result = black_box(controller.run_large());
        })
    });

    // Passing Boxed structs
    c.bench_function("box_small", |b| {
        b.iter(|| {
            let boxed_small = black_box(controller.run_small_boxed());
        })
    });

    c.bench_function("box_medium", |b| {
        b.iter(|| {
            let boxed_medium = black_box(controller.run_medium_boxed());
        })
    });

    c.bench_function("box_large", |b| {
        b.iter(|| {
            let boxed_large = black_box(controller.run_large_boxed());
        })
    });
}

criterion_group!(benches, benchmark_3layer_architecture);
criterion_main!(benches);
