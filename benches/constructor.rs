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

// Constructors by move accepting all arguments
fn construct_small_move(a: u32, b: String, c: f64) -> Small {
    Small { a, b, c }
}

fn construct_medium_move(
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
    }
}

fn construct_large_move(
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

// New functions returning Boxed structs accepting all arguments
fn construct_small_boxed(a: u32, b: String, c: f64) -> Box<Small> {
    Box::new(Small { a, b, c })
}

fn construct_medium_boxed(
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
    })
}

fn construct_large_boxed(
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
            let _ = black_box(construct_medium_move(
                1,
                "text".into(),
                3.14,
                vec![1, 2, 3],
                HashMap::new(),
                true,
                Some(42),
                "text".to_string(),
                2.71,
                vec!["foo".into(), "bar".into()],
            ));
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
            let _ = black_box(construct_large_move(
                1,
                2,
                3,
                4,
                [0; 1024],
                [0; 512],
                vec!["hello".into()],
                HashMap::new(),
                1.23,
                3.21,
                true,
                vec![1, 2, 3],
                Some(5),
                "n".into(),
                "o".into(),
                "p".into(),
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19,
                20,
                21,
            ));
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

    // New benchmarks with Boxed structs accepting all arguments
    c.bench_function("construct_small_boxed", |b| {
        b.iter(|| {
            let _ = black_box(construct_small_boxed(1, "text".into(), 3.14));
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
                true,
                Some(42),
                "text".to_string(),
                2.71,
                vec!["foo".into(), "bar".into()],
            ));
        })
    });

    c.bench_function("construct_large_boxed", |b| {
        b.iter(|| {
            let _ = black_box(construct_large_boxed(
                1,
                2,
                3,
                4,
                [0; 1024],
                [0; 512],
                vec!["hello".into()],
                HashMap::new(),
                1.23,
                3.21,
                true,
                vec![1, 2, 3],
                Some(5),
                "n".into(),
                "o".into(),
                "p".into(),
                6,
                7,
                8,
                9,
                10,
                11,
                12,
                13,
                14,
                15,
                16,
                17,
                18,
                19,
                20,
                21,
            ));
        })
    });
}

criterion_group!(benches, benchmark_constructors);
criterion_main!(benches);
