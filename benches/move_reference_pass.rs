use criterion::{black_box, Criterion, criterion_group, criterion_main};
use std::collections::HashMap;

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

// Reference functions
fn pass_by_reference_small(small: &Small) {
    black_box(small);
}

fn pass_by_reference_medium(medium: &Medium) {
    black_box(medium);
}

fn pass_by_reference_large(large: &Large) {
    black_box(large);
}

// Mutable reference functions
fn pass_by_mutable_reference_small(small: &mut Small) {
    small.a += 1;
}

fn pass_by_mutable_reference_medium(medium: &mut Medium) {
    medium.a += 1;
}

fn pass_by_mutable_reference_large(large: &mut Large) {
    large.a += 1;
}

// Move functions
fn move_small(small: Small) -> Small {
    small
}

fn move_medium(medium: Medium) -> Medium {
    medium
}

fn move_large(large: Large) -> Large {
    large
}

// Clone functions
fn clone_small(small: &Small) -> Small {
    small.clone()
}

fn clone_medium(medium: &Medium) -> Medium {
    medium.clone()
}

fn clone_large(large: &Large) -> Large {
    large.clone()
}

// Benchmark suite
fn benchmark(c: &mut Criterion) {
    // Small
    c.bench_function("pass_by_reference_small", |b| {
        b.iter(|| {
            let small = Small {
                a: 1,
                b: String::from("small"),
                c: 1.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(1),
                h: vec![String::from("h1")],
                i: 1.0,
                j: HashMap::new(),
            };
            pass_by_reference_small(&small)
        })
    });

    c.bench_function("pass_by_mutable_reference_small", |b| {
        b.iter(|| {
            let mut small = Small {
                a: 1,
                b: String::from("small"),
                c: 1.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(1),
                h: vec![String::from("h1")],
                i: 1.0,
                j: HashMap::new(),
            };
            pass_by_mutable_reference_small(&mut small)
        })
    });

    c.bench_function("move_small", |b| {
        b.iter(|| {
            let small = Small {
                a: 1,
                b: String::from("small"),
                c: 1.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(1),
                h: vec![String::from("h1")],
                i: 1.0,
                j: HashMap::new(),
            };
            move_small(small)
        })
    });

    c.bench_function("clone_small", |b| {
        b.iter(|| {
            let small = Small {
                a: 1,
                b: String::from("small"),
                c: 1.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(1),
                h: vec![String::from("h1")],
                i: 1.0,
                j: HashMap::new(),
            };
            clone_small(&small)
        })
    });

    // Medium
    c.bench_function("pass_by_reference_medium", |b| {
        b.iter(|| {
            let medium = Medium {
                a: 1,
                b: String::from("medium"),
                c: 2.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: 1,
                g: vec![1],
                h: String::from("h2"),
                i: HashMap::new(),
                j: 2.0,
                k: Some(1),
                l: vec![1.0],
                m: true,
                n: vec![String::from("n1")],
                o: HashMap::new(),
                p: String::from("p"),
                q: None,
                r: vec![1],
                s: HashMap::new(),
                t: 3.0,
            };
            pass_by_reference_medium(&medium)
        })
    });

    c.bench_function("pass_by_mutable_reference_medium", |b| {
        b.iter(|| {
            let mut medium = Medium {
                a: 1,
                b: String::from("medium"),
                c: 2.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: 1,
                g: vec![1],
                h: String::from("h2"),
                i: HashMap::new(),
                j: 2.0,
                k: Some(1),
                l: vec![1.0],
                m: true,
                n: vec![String::from("n1")],
                o: HashMap::new(),
                p: String::from("p"),
                q: None,
                r: vec![1],
                s: HashMap::new(),
                t: 3.0,
            };
            pass_by_mutable_reference_medium(&mut medium)
        })
    });

    c.bench_function("move_medium", |b| {
        b.iter(|| {
            let medium = Medium {
                a: 1,
                b: String::from("medium"),
                c: 2.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: 1,
                g: vec![1],
                h: String::from("h2"),
                i: HashMap::new(),
                j: 2.0,
                k: Some(1),
                l: vec![1.0],
                m: true,
                n: vec![String::from("n1")],
                o: HashMap::new(),
                p: String::from("p"),
                q: None,
                r: vec![1],
                s: HashMap::new(),
                t: 3.0,
            };
            move_medium(medium)
        })
    });

    c.bench_function("clone_medium", |b| {
        b.iter(|| {
            let medium = Medium {
                a: 1,
                b: String::from("medium"),
                c: 2.0,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: 1,
                g: vec![1],
                h: String::from("h2"),
                i: HashMap::new(),
                j: 2.0,
                k: Some(1),
                l: vec![1.0],
                m: true,
                n: vec![String::from("n1")],
                o: HashMap::new(),
                p: String::from("p"),
                q: None,
                r: vec![1],
                s: HashMap::new(),
                t: 3.0,
            };
            clone_medium(&medium)
        })
    });

    // Large
    c.bench_function("pass_by_reference_large", |b| {
        b.iter(|| {
            let large = Large {
                a: 1,
                b: 1.0,
                c: String::from("large"),
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: None,
                g: 2,
                h: vec![1],
                i: HashMap::new(),
                j: 2.0,
                k: 1,
                l: vec![1.0],
                m: String::from("m"),
                n: HashMap::new(),
                o: vec![String::from("o1")],
                p: vec![1],
                q: vec![1],
                r: HashMap::new(),
                s: vec![1],
                t: Some(vec![1]),
                u: 2.0,
                v: String::from("v"),
                w: 3,
                x: 4.0,
                y: 5,
                z: String::from("z"),
                aa: vec![1.0],
                ab: 6,
                ac: HashMap::new(),
                ad: None,
                ae: vec![String::from("ae1")],
                af: HashMap::new(),
            };
            pass_by_reference_large(&large)
        })
    });

    c.bench_function("pass_by_mutable_reference_large", |b| {
        b.iter(|| {
            let mut large = Large {
                a: 1,
                b: 1.0,
                c: String::from("large"),
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: None,
                g: 2,
                h: vec![1],
                i: HashMap::new(),
                j: 2.0,
                k: 1,
                l: vec![1.0],
                m: String::from("m"),
                n: HashMap::new(),
                o: vec![String::from("o1")],
                p: vec![1],
                q: vec![1],
                r: HashMap::new(),
                s: vec![1],
                t: Some(vec![1]),
                u: 2.0,
                v: String::from("v"),
                w: 3,
                x: 4.0,
                y: 5,
                z: String::from("z"),
                aa: vec![1.0],
                ab: 6,
                ac: HashMap::new(),
                ad: None,
                ae: vec![String::from("ae1")],
                af: HashMap::new(),
            };
            pass_by_mutable_reference_large(&mut large)
        })
    });

    c.bench_function("move_large", |b| {
        b.iter(|| {
            let large = Large {
                a: 1,
                b: 1.0,
                c: String::from("large"),
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: None,
                g: 2,
                h: vec![1],
                i: HashMap::new(),
                j: 2.0,
                k: 1,
                l: vec![1.0],
                m: String::from("m"),
                n: HashMap::new(),
                o: vec![String::from("o1")],
                p: vec![1],
                q: vec![1],
                r: HashMap::new(),
                s: vec![1],
                t: Some(vec![1]),
                u: 2.0,
                v: String::from("v"),
                w: 3,
                x: 4.0,
                y: 5,
                z: String::from("z"),
                aa: vec![1.0],
                ab: 6,
                ac: HashMap::new(),
                ad: None,
                ae: vec![String::from("ae1")],
                af: HashMap::new(),
            };
            move_large(large)
        })
    });

    c.bench_function("clone_large", |b| {
        b.iter(|| {
            let large = Large {
                a: 1,
                b: 1.0,
                c: String::from("large"),
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: None,
                g: 2,
                h: vec![1],
                i: HashMap::new(),
                j: 2.0,
                k: 1,
                l: vec![1.0],
                m: String::from("m"),
                n: HashMap::new(),
                o: vec![String::from("o1")],
                p: vec![1],
                q: vec![1],
                r: HashMap::new(),
                s: vec![1],
                t: Some(vec![1]),
                u: 2.0,
                v: String::from("v"),
                w: 3,
                x: 4.0,
                y: 5,
                z: String::from("z"),
                aa: vec![1.0],
                ab: 6,
                ac: HashMap::new(),
                ad: None,
                ae: vec![String::from("ae1")],
                af: HashMap::new(),
            };
            clone_large(&large)
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
