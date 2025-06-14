use criterion::{Criterion, black_box};
use std::collections::HashMap;

#[derive(Clone)]
struct Small {
    a: i32,
    b: String,
    c: f64,
    d: Vec<i32>,
    e: HashMap<String, i32>,
    f: bool,
    g: Option<i32>,
    h: Vec<String>,
    i: f64,
    j: HashMap<String, i32>,
}

#[derive(Clone)]
struct SmallV2 {
    a: i32,
    b: String,
    c: f64,
    d: Vec<i32>,
    e: HashMap<String, i32>,
    f: bool,
    g: Option<i32>,
    h: Vec<String>,
    i: f64,
    j: HashMap<String, i32>,
}

// Map and return mapped struct for Small
fn map_small(input: Small) -> SmallV2 {
    SmallV2 {
        a: input.a + 1,
        b: input.b,
        c: input.c + 1.0,
        d: input.d,
        e: input.e,
        f: input.f,
        g: input.g.map(|v| v + 1),
        h: input.h,
        i: input.i + 1.0,
        j: input.j,
    }
}

// Just return the input without any changes for Small
fn return_small(input: Small) -> Small {
    input
}


#[derive(Clone)]
struct Medium {
    a: i32,
    b: String,
    c: f64,
    d: Vec<i32>,
    e: HashMap<String, i32>,
    f: bool,
    g: Option<i32>,
    h: Vec<String>,
    i: f64,
    j: HashMap<String, i32>,

    k: String,
    l: Vec<u8>,
    m: Option<String>,
    n: HashMap<String, String>,
    o: bool,
}

#[derive(Clone)]
struct MediumV2 {
    a: i32,
    b: String,
    c: f64,
    d: Vec<i32>,
    e: HashMap<String, i32>,
    f: bool,
    g: Option<i32>,
    h: Vec<String>,
    i: f64,
    j: HashMap<String, i32>,

    k: String,
    l: Vec<u8>,
    m: Option<String>,
    n: HashMap<String, String>,
    o: bool,
}

// Map and return mapped struct for Medium
fn map_medium(input: Medium) -> MediumV2 {
    MediumV2 {
        a: input.a + 1,
        b: input.b,
        c: input.c + 1.0,
        d: input.d,
        e: input.e,
        f: input.f,
        g: input.g.map(|v| v + 1),
        h: input.h,
        i: input.i + 1.0,
        j: input.j,

        k: input.k,
        l: input.l,
        m: input.m,
        n: input.n,
        o: input.o,
    }
}

// Just return the input without any changes for Medium
fn return_medium(input: Medium) -> Medium {
    input
}


#[derive(Clone)]
struct Large {
    a: i32,
    b: String,
    c: f64,
    d: Vec<i32>,
    e: HashMap<String, i32>,
    f: bool,
    g: Option<i32>,
    h: Vec<String>,
    i: f64,
    j: HashMap<String, i32>,

    k: String,
    l: Vec<u8>,
    m: Option<String>,
    n: HashMap<String, String>,
    o: bool,

    p: Vec<i64>,
    q: Option<HashMap<String, i32>>,
    r: Vec<String>,
    s: bool,
    t: f64,
}

#[derive(Clone)]
struct LargeV2 {
    a: i32,
    b: String,
    c: f64,
    d: Vec<i32>,
    e: HashMap<String, i32>,
    f: bool,
    g: Option<i32>,
    h: Vec<String>,
    i: f64,
    j: HashMap<String, i32>,

    k: String,
    l: Vec<u8>,
    m: Option<String>,
    n: HashMap<String, String>,
    o: bool,

    p: Vec<i64>,
    q: Option<HashMap<String, i32>>,
    r: Vec<String>,
    s: bool,
    t: f64,
}

// Map and return mapped struct for Large
fn map_large(input: Large) -> LargeV2 {
    LargeV2 {
        a: input.a + 1,
        b: input.b,
        c: input.c + 1.0,
        d: input.d,
        e: input.e,
        f: input.f,
        g: input.g.map(|v| v + 1),
        h: input.h,
        i: input.i + 1.0,
        j: input.j,

        k: input.k,
        l: input.l,
        m: input.m,
        n: input.n,
        o: input.o,

        p: input.p,
        q: input.q,
        r: input.r,
        s: input.s,
        t: input.t + 1.0,
    }
}

// Just return the input without any changes for Large
fn return_large(input: Large) -> Large {
    input
}


fn bench_small(c: &mut Criterion) {
    let mut small_map = HashMap::new();
    small_map.insert("key".to_string(), 42);
    let small = Small {
        a: 1,
        b: "small".into(),
        c: 1.0,
        d: vec![1, 2, 3],
        e: small_map.clone(),
        f: true,
        g: Some(10),
        h: vec!["hello".into()],
        i: 3.14,
        j: small_map,
    };

    c.bench_function("small_map_and_return", |b| {
        b.iter(|| {
            let cloned = small.clone();
            let out = map_small(black_box(cloned));
            black_box(out);
        })
    });

    c.bench_function("small_return_input", |b| {
        b.iter(|| {
            let cloned = small.clone();
            let out = return_small(black_box(cloned));
            black_box(out);
        })
    });
}

fn bench_medium(c: &mut Criterion) {
    let mut map_i32 = HashMap::new();
    map_i32.insert("key".to_string(), 42);

    let mut map_str = HashMap::new();
    map_str.insert("hello".to_string(), "world".to_string());

    let medium = Medium {
        a: 1,
        b: "medium".into(),
        c: 2.0,
        d: vec![4, 5, 6],
        e: map_i32.clone(),
        f: false,
        g: Some(20),
        h: vec!["world".into()],
        i: 6.28,
        j: map_i32.clone(),

        k: "extra".into(),
        l: vec![1, 2, 3, 4],
        m: Some("option".into()),
        n: map_str,
        o: true,
    };

    c.bench_function("medium_map_and_return", |b| {
        b.iter(|| {
            let cloned = medium.clone();
            let out = map_medium(black_box(cloned));
            black_box(out);
        })
    });

    c.bench_function("medium_return_input", |b| {
        b.iter(|| {
            let cloned = medium.clone();
            let out = return_medium(black_box(cloned));
            black_box(out);
        })
    });
}

fn bench_large(c: &mut Criterion) {
    let mut map_i32 = HashMap::new();
    map_i32.insert("key".to_string(), 42);

    let mut map_str = HashMap::new();
    map_str.insert("hello".to_string(), "world".to_string());

    let large = Large {
        a: 1,
        b: "large".into(),
        c: 3.0,
        d: vec![7, 8, 9],
        e: map_i32.clone(),
        f: true,
        g: Some(30),
        h: vec!["large".into()],
        i: 9.42,
        j: map_i32.clone(),

        k: "bigger".into(),
        l: vec![5, 6, 7, 8],
        m: Some("some option".into()),
        n: map_str.clone(),
        o: false,

        p: vec![10, 20, 30],
        q: Some(map_i32),
        r: vec!["string1".into(), "string2".into()],
        s: true,
        t: 42.0,
    };

    c.bench_function("large_map_and_return", |b| {
        b.iter(|| {
            let cloned = large.clone();
            let out = map_large(black_box(cloned));
            black_box(out);
        })
    });

    c.bench_function("large_return_input", |b| {
        b.iter(|| {
            let cloned = large.clone();
            let out = return_large(black_box(cloned));
            black_box(out);
        })
    });
}

criterion::criterion_group!(benches, bench_small, bench_medium, bench_large);
criterion::criterion_main!(benches);
