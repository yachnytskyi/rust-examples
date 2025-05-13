use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;

#[derive(Clone)]
struct Small {
    a: u32,
    b: String,
    c: f64,
}

#[derive(Clone)]
struct Medium {
    a: u32,
    b: String,
    c: f64,
    d: Vec<u8>,
    e: HashMap<String, u32>,
    f: bool,
    g: Option<u64>,
    h: String,
    i: f64,
    j: Vec<String>,
}

#[derive(Clone)]
struct Large {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: [u8; 1024],
    f: [u32; 512],
    g: Vec<String>,
    h: HashMap<String, String>,
    i: f64,
    j: f64,
    k: bool,
    l: Vec<u64>,
    m: Option<u64>,
    n: String,
    o: String,
    p: String,
    q: u64,
    r: u64,
    s: u64,
    t: u64,
    u: u64,
    v: u64,
    w: u64,
    x: u64,
    y: u64,
    z: u64,
    aa: u64,
    ab: u64,
    ac: u64,
    ad: u64,
    ae: u64,
    af: u64,
}

// Repository Layer: Handles data access and returns structs (Boxed or Owned)
struct Repository;

impl Repository {
    // Returns a moved Small struct
    fn get_small(&self) -> Small {
        Small { a: 1, b: "text".into(), c: 3.14 }
    }

    // Returns a moved Medium struct
    fn get_medium(&self) -> Medium {
        Medium {
            a: 1,
            b: "text".into(),
            c: 3.14,
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: true,
            g: Some(42),
            h: "text".into(),
            i: 2.71,
            j: vec!["foo".into(), "bar".into()],
        }
    }

    // Returns a moved Large struct
    fn get_large(&self) -> Large {
        Large {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: [0; 1024],
            f: [0; 512],
            g: vec!["hello".into()],
            h: HashMap::new(),
            i: 1.23,
            j: 3.21,
            k: true,
            l: vec![1, 2, 3],
            m: Some(5),
            n: "n".into(),
            o: "o".into(),
            p: "p".into(),
            q: 6,
            r: 7,
            s: 8,
            t: 9,
            u: 10,
            v: 11,
            w: 12,
            x: 13,
            y: 14,
            z: 15,
            aa: 16,
            ab: 17,
            ac: 18,
            ad: 19,
            ae: 20,
            af: 21,
        }
    }

    // Returns a Boxed Small struct
    fn get_small_boxed(&self) -> Box<Small> {
        Box::new(Small { a: 1, b: "text".into(), c: 3.14 })
    }

    // Returns a Boxed Medium struct
    fn get_medium_boxed(&self) -> Box<Medium> {
        Box::new(Medium {
            a: 1,
            b: "text".into(),
            c: 3.14,
            d: vec![1, 2, 3],
            e: HashMap::new(),
            f: true,
            g: Some(42),
            h: "text".into(),
            i: 2.71,
            j: vec!["foo".into(), "bar".into()],
        })
    }

    // Returns a Boxed Large struct
    fn get_large_boxed(&self) -> Box<Large> {
        Box::new(Large {
            a: 1,
            b: 2,
            c: 3,
            d: 4,
            e: [0; 1024],
            f: [0; 512],
            g: vec!["hello".into()],
            h: HashMap::new(),
            i: 1.23,
            j: 3.21,
            k: true,
            l: vec![1, 2, 3],
            m: Some(5),
            n: "n".into(),
            o: "o".into(),
            p: "p".into(),
            q: 6,
            r: 7,
            s: 8,
            t: 9,
            u: 10,
            v: 11,
            w: 12,
            x: 13,
            y: 14,
            z: 15,
            aa: 16,
            ab: 17,
            ac: 18,
            ad: 19,
            ae: 20,
            af: 21,
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
