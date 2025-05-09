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

// Constructors by move
fn construct_small_move(a: u32, b: String, c: f64) -> Small {
    Small { a, b, c }
}

fn construct_medium_move(a: u32, b: String, c: f64) -> Medium {
    Medium {
        a,
        b,
        c,
        d: vec![1, 2, 3],
        e: HashMap::new(),
        f: true,
        g: Some(42),
        h: "text".to_string(),
        i: 2.71,
        j: vec!["foo".into(), "bar".into()],
    }
}

fn construct_large_move(a: u64, b: u64) -> Large {
    Large {
        a,
        b,
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

// Benchmark group
fn benchmark_constructors(c: &mut Criterion) {
    // Small
    c.bench_function("construct_small_move", |b| {
        b.iter(|| {
            let _ = black_box(construct_small_move(1, "text".into(), 3.14));
        })
    });

    c.bench_function("construct_small_direct", |b| {
        b.iter(|| {
            let _ = black_box(Small {
                a: 1,
                b: "text".into(),
                c: 3.14,
            });
        })
    });

    // Medium
    c.bench_function("construct_medium_move", |b| {
        b.iter(|| {
            let _ = black_box(construct_medium_move(1, "text".into(), 3.14));
        })
    });

    c.bench_function("construct_medium_direct", |b| {
        b.iter(|| {
            let _ = black_box(Medium {
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
            });
        })
    });

    // Large
    c.bench_function("construct_large_move", |b| {
        b.iter(|| {
            let _ = black_box(construct_large_move(1, 2));
        })
    });

    c.bench_function("construct_large_direct", |b| {
        b.iter(|| {
            let _ = black_box(Large {
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
            });
        })
    });
}

criterion_group!(benches, benchmark_constructors);
criterion_main!(benches);
