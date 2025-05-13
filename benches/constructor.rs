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

// Constructors by move accepting all arguments
fn construct_small_move(
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
) -> Small {
    Small {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
    }
}

fn construct_medium_move(
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
) -> Medium {
    Medium {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n,
        o,
        p,
        q,
        r,
        s,
        t,
    }
}

fn construct_large_move(
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
) -> Large {
    Large {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n,
        o,
        p,
        q,
        r,
        s,
        t,
        u,
        v,
        w,
        x,
        y,
        z,
        aa,
        ab,
        ac,
        ad,
        ae,
        af,
    }
}

// Boxed constructors
fn construct_small_boxed(
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
) -> Box<Small> {
    Box::new(Small {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
    })
}

fn construct_medium_boxed(
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
) -> Box<Medium> {
    Box::new(Medium {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n,
        o,
        p,
        q,
        r,
        s,
        t,
    })
}

fn construct_large_boxed(
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
) -> Box<Large> {
    Box::new(Large {
        a,
        b,
        c,
        d,
        e,
        f,
        g,
        h,
        i,
        j,
        k,
        l,
        m,
        n,
        o,
        p,
        q,
        r,
        s,
        t,
        u,
        v,
        w,
        x,
        y,
        z,
        aa,
        ab,
        ac,
        ad,
        ae,
        af,
    })
}

fn benchmark_constructors(c: &mut Criterion) {
    // Small struct benchmarks
    c.bench_function("construct_small_direct", |b| {
        b.iter(|| {
            let _ = black_box(Small {
                a: 1,
                b: "text".into(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: vec!["world".to_string()],
                i: 2.5,
                j: HashMap::new(),
            });
        })
    });

    c.bench_function("construct_small_move", |b| {
        b.iter(|| {
            let _ = black_box(construct_small_move(
                1,
                "text".into(),
                3.14,
                vec![1, 2, 3],
                HashMap::new(),
                true,
                Some(42),
                vec!["world".to_string()],
                2.5,
                HashMap::new(),
            ));
        })
    });

    c.bench_function("construct_small_boxed", |b| {
        b.iter(|| {
            let _ = black_box(construct_small_boxed(
                1,
                "text".into(),
                3.14,
                vec![1, 2, 3],
                HashMap::new(),
                true,
                Some(42),
                vec!["world".to_string()],
                2.5,
                HashMap::new(),
            ));
        })
    });

    // Medium struct benchmarks
    c.bench_function("construct_medium_direct", |b| {
        b.iter(|| {
            let _ = black_box(Medium {
                a: 1,
                b: "text".into(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: 64,
                g: vec![1, 2, 3],
                h: "text".into(),
                i: HashMap::new(),
                j: 2.5,
                k: Some(42),
                l: vec![1.1, 2.2],
                m: true,
                n: vec!["one".into(), "two".into()],
                o: HashMap::new(),
                p: "text".into(),
                q: Some(vec![1, 2, 3]),
                r: vec![1, 2],
                s: HashMap::new(),
                t: 3.14,
            });
        })
    });

    c.bench_function("construct_medium_move", |b| {
        b.iter(|| {
            let _ = black_box(construct_medium_move(
                1,
                "text".into(),
                3.14,
                vec![1, 2, 3],
                HashMap::new(),
                64,
                vec![1, 2, 3],
                "text".into(),
                HashMap::new(),
                2.5,
                Some(42),
                vec![1.1, 2.2],
                true,
                vec!["one".into(), "two".into()],
                HashMap::new(),
                "text".into(),
                Some(vec![1, 2, 3]),
                vec![1, 2],
                HashMap::new(),
                3.14,
            ));
        })
    });

    c.bench_function("construct_medium_boxed", |b| {
        b.iter(|| {
            let _ = black_box(construct_medium_boxed(
                1,
                "text".into(),
                3.14,
                vec![1, 2, 3],
                HashMap::new(),
                64,
                vec![1, 2, 3],
                "text".into(),
                HashMap::new(),
                2.5,
                Some(42),
                vec![1.1, 2.2],
                true,
                vec!["one".into(), "two".into()],
                HashMap::new(),
                "text".into(),
                Some(vec![1, 2, 3]),
                vec![1, 2],
                HashMap::new(),
                3.14,
            ));
        })
    });

    // Large struct benchmarks
    c.bench_function("construct_large_direct", |b| {
        b.iter(|| {
            let _ = black_box(Large {
                a: 1000,
                b: 3.14,
                c: "Large".to_string(),
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: Some(42),
                g: 12345,
                h: vec![10, 20, 30],
                i: HashMap::new(),
                j: 6.28,
                k: 99,
                l: vec![1.1, 2.2, 3.3],
                m: "Hello".to_string(),
                n: HashMap::new(),
                o: vec!["a".to_string(), "b".to_string()],
                p: vec![100, 200],
                q: vec![1, 2, 3],
                r: HashMap::new(),
                s: vec![10, 20, 30],
                t: Some(vec![4, 5, 6]),
                u: 9.81,
                v: "world".to_string(),
                w: 42,
                x: 2.71,
                y: 98765,
                z: "end".to_string(),
                aa: vec![1.0, 2.0, 3.0],
                ab: 100000,
                ac: HashMap::new(),
                ad: Some("optional".to_string()),
                ae: vec!["test".to_string()],
                af: HashMap::new(),
            });
        })
    });

    c.bench_function("construct_large_move", |b| {
        b.iter(|| {
            let _ = black_box(construct_large_move(
                1000,
                3.14,
                "Large".to_string(),
                vec![1, 2, 3],
                HashMap::new(),
                Some(42),
                12345,
                vec![10, 20, 30],
                HashMap::new(),
                6.28,
                99,
                vec![1.1, 2.2, 3.3],
                "Hello".to_string(),
                HashMap::new(),
                vec!["a".to_string(), "b".to_string()],
                vec![100, 200],
                vec![1, 2, 3],
                HashMap::new(),
                vec![10, 20, 30],
                Some(vec![4, 5, 6]),
                9.81,
                "world".to_string(),
                42,
                2.71,
                98765,
                "end".to_string(),
                vec![1.0, 2.0, 3.0],
                100000,
                HashMap::new(),
                Some("optional".to_string()),
                vec!["test".to_string()],
                HashMap::new(),
            ));
        })
    });

    c.bench_function("construct_large_boxed", |b| {
        b.iter(|| {
            let _ = black_box(construct_large_boxed(
                1000,
                3.14,
                "Large".to_string(),
                vec![1, 2, 3],
                HashMap::new(),
                Some(42),
                12345,
                vec![10, 20, 30],
                HashMap::new(),
                6.28,
                99,
                vec![1.1, 2.2, 3.3],
                "Hello".to_string(),
                HashMap::new(),
                vec!["a".to_string(), "b".to_string()],
                vec![100, 200],
                vec![1, 2, 3],
                HashMap::new(),
                vec![10, 20, 30],
                Some(vec![4, 5, 6]),
                9.81,
                "world".to_string(),
                42,
                2.71,
                98765,
                "end".to_string(),
                vec![1.0, 2.0, 3.0],
                100000,
                HashMap::new(),
                Some("optional".to_string()),
                vec!["test".to_string()],
                HashMap::new(),
            ));
        })
    });
}

criterion_group!(benches, benchmark_constructors);
criterion_main!(benches);
