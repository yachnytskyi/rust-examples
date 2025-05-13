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
struct SmallV2 {
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
struct MediumV2 {
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

#[derive(Clone)]
struct LargeV2 {
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

// --- MAPPING FUNCTIONS ---

// Move
fn map_small_to_v2_move(input: Small) -> SmallV2 {
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

fn map_medium_to_v2_move(input: Medium) -> MediumV2 {
    MediumV2 {
        a: input.a + 1,
        b: input.b,
        c: input.c + 1.0,
        d: input.d,
        e: input.e,
        f: input.f + 1,
        g: input.g,
        h: input.h,
        i: input.i,
        j: input.j + 1.0,
        k: input.k.map(|v| v + 1),
        l: input.l,
        m: !input.m,
        n: input.n,
        o: input.o,
        p: input.p,
        q: input.q,
        r: input.r,
        s: input.s,
        t: input.t + 1.0,
    }
}

fn map_large_to_v2_move(input: Large) -> LargeV2 {
    LargeV2 {
        a: input.a + 1,
        b: input.b + 1.0,
        c: input.c,
        d: input.d,
        e: input.e,
        f: input.f.map(|v| v + 1),
        g: input.g + 1,
        h: input.h,
        i: input.i,
        j: input.j + 1.0,
        k: input.k + 1,
        l: input.l,
        m: input.m,
        n: input.n,
        o: input.o,
        p: input.p,
        q: input.q,
        r: input.r,
        s: input.s,
        t: input.t,
        u: input.u + 1.0,
        v: input.v,
        w: input.w + 1,
        x: input.x + 1.0,
        y: input.y + 1,
        z: input.z,
        aa: input.aa,
        ab: input.ab + 1,
        ac: input.ac,
        ad: input.ad,
        ae: input.ae,
        af: input.af,
    }
}

// Clone
fn map_small_to_v2_clone(input: Small) -> SmallV2 {
    SmallV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
        d: input.d.clone(),
        e: input.e.clone(),
        f: input.f,
        g: input.g.map(|v| v + 1),
        h: input.h.clone(),
        i: input.i + 1.0,
        j: input.j.clone(),
    }
}

fn map_medium_to_v2_clone(input: Medium) -> MediumV2 {
    MediumV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
        d: input.d.clone(),
        e: input.e.clone(),
        f: input.f + 1,
        g: input.g.clone(),
        h: input.h.clone(),
        i: input.i.clone(),
        j: input.j + 1.0,
        k: input.k.map(|v| v + 1),
        l: input.l.clone(),
        m: !input.m,
        n: input.n.clone(),
        o: input.o.clone(),
        p: input.p.clone(),
        q: input.q.clone(),
        r: input.r.clone(),
        s: input.s.clone(),
        t: input.t + 1.0,
    }
}

fn map_large_to_v2_clone(input: Large) -> LargeV2 {
    LargeV2 {
        a: input.a + 1,
        b: input.b + 1.0,
        c: input.c.clone(),
        d: input.d.clone(),
        e: input.e.clone(),
        f: input.f.map(|v| v + 1),
        g: input.g + 1,
        h: input.h.clone(),
        i: input.i.clone(),
        j: input.j + 1.0,
        k: input.k + 1,
        l: input.l.clone(),
        m: input.m.clone(),
        n: input.n.clone(),
        o: input.o.clone(),
        p: input.p.clone(),
        q: input.q.clone(),
        r: input.r.clone(),
        s: input.s.clone(),
        t: input.t.clone(),
        u: input.u + 1.0,
        v: input.v.clone(),
        w: input.w + 1,
        x: input.x + 1.0,
        y: input.y + 1,
        z: input.z.clone(),
        aa: input.aa.clone(),
        ab: input.ab + 1,
        ac: input.ac.clone(),
        ad: input.ad.clone(),
        ae: input.ae.clone(),
        af: input.af.clone(),
    }
}

// Reference
fn map_small_to_v2_reference(input: &Small) -> SmallV2 {
    SmallV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
        d: input.d.clone(),
        e: input.e.clone(),
        f: input.f,
        g: input.g.map(|v| v + 1),
        h: input.h.clone(),
        i: input.i + 1.0,
        j: input.j.clone(),
    }
}

fn map_medium_to_v2_reference(input: &Medium) -> MediumV2 {
    MediumV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
        d: input.d.clone(),
        e: input.e.clone(),
        f: input.f + 1,
        g: input.g.clone(),
        h: input.h.clone(),
        i: input.i.clone(),
        j: input.j + 1.0,
        k: input.k.map(|v| v + 1),
        l: input.l.clone(),
        m: !input.m,
        n: input.n.clone(),
        o: input.o.clone(),
        p: input.p.clone(),
        q: input.q.clone(),
        r: input.r.clone(),
        s: input.s.clone(),
        t: input.t + 1.0,
    }
}

fn map_large_to_v2_reference(input: &Large) -> LargeV2 {
    LargeV2 {
        a: input.a + 1,
        b: input.b + 1.0,
        c: input.c.clone(),
        d: input.d.clone(),
        e: input.e.clone(),
        f: input.f.map(|v| v + 1),
        g: input.g + 1,
        h: input.h.clone(),
        i: input.i.clone(),
        j: input.j + 1.0,
        k: input.k + 1,
        l: input.l.clone(),
        m: input.m.clone(),
        n: input.n.clone(),
        o: input.o.clone(),
        p: input.p.clone(),
        q: input.q.clone(),
        r: input.r.clone(),
        s: input.s.clone(),
        t: input.t.clone(),
        u: input.u + 1.0,
        v: input.v.clone(),
        w: input.w + 1,
        x: input.x + 1.0,
        y: input.y + 1,
        z: input.z.clone(),
        aa: input.aa.clone(),
        ab: input.ab + 1,
        ac: input.ac.clone(),
        ad: input.ad.clone(),
        ae: input.ae.clone(),
        af: input.af.clone(),
    }
}

// --- BENCHMARKS ---

fn benchmark_mapping(c: &mut Criterion) {
    // Benchmark for Small -> SmallV2 (Move)
    c.bench_function("map_small_to_v2_move", |b| {
        b.iter(|| {
            let s = Small {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: vec!["world".to_string()],
                i: 2.5,
                j: HashMap::new(),
            };
            let _ = black_box(map_small_to_v2_move(s));
        })
    });

    // Benchmark for Medium -> MediumV2 (Move)
    c.bench_function("map_medium_to_v2_move", |b| {
        b.iter(|| {
            let m = Medium {
                a: 1,
                b: "Medium".to_string(),
                c: 6.28,
                d: vec![4, 5, 6],
                e: HashMap::new(),
                f: 64,
                g: vec![10, 20, 30],
                h: "medium".to_string(),
                i: HashMap::new(),
                j: 1.23,
                k: Some(99),
                l: vec![1.1, 2.2, 3.3],
                m: false,
                n: vec!["one".to_string(), "two".to_string()],
                o: HashMap::new(),
                p: "p".to_string(),
                q: Some(vec![7, 8, 9]),
                r: vec![100, 200],
                s: HashMap::new(),
                t: 9.81,
            };
            let _ = black_box(map_medium_to_v2_move(m));
        })
    });

    // Benchmark for Large -> LargeV2 (Move)
    c.bench_function("map_large_to_v2_move", |b| {
        b.iter(|| {
            let l = Large {
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
            };
            let _ = black_box(map_large_to_v2_move(l));
        })
    });

    // Clone benchmarks
    c.bench_function("map_small_to_v2_clone", |b| {
        b.iter(|| {
            let s = Small {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: vec!["world".to_string()],
                i: 2.5,
                j: HashMap::new(),
            };
            let _ = black_box(map_small_to_v2_clone(s));
        })
    });

    c.bench_function("map_medium_to_v2_clone", |b| {
        b.iter(|| {
            let m = Medium {
                a: 1,
                b: "Medium".to_string(),
                c: 6.28,
                d: vec![4, 5, 6],
                e: HashMap::new(),
                f: 64,
                g: vec![10, 20, 30],
                h: "medium".to_string(),
                i: HashMap::new(),
                j: 1.23,
                k: Some(99),
                l: vec![1.1, 2.2, 3.3],
                m: false,
                n: vec!["one".to_string(), "two".to_string()],
                o: HashMap::new(),
                p: "p".to_string(),
                q: Some(vec![7, 8, 9]),
                r: vec![100, 200],
                s: HashMap::new(),
                t: 9.81,
            };
            let _ = black_box(map_medium_to_v2_clone(m));
        })
    });

    c.bench_function("map_large_to_v2_clone", |b| {
        b.iter(|| {
            let l = Large {
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
            };
            let _ = black_box(map_large_to_v2_clone(l));
        })
    });

    // Reference benchmarks
    c.bench_function("map_small_to_v2_reference", |b| {
        b.iter(|| {
            let s = Small {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: vec!["world".to_string()],
                i: 2.5,
                j: HashMap::new(),
            };
            let _ = black_box(map_small_to_v2_reference(&s));
        })
    });

    c.bench_function("map_medium_to_v2_reference", |b| {
        b.iter(|| {
            let m = Medium {
                a: 1,
                b: "Medium".to_string(),
                c: 6.28,
                d: vec![4, 5, 6],
                e: HashMap::new(),
                f: 64,
                g: vec![10, 20, 30],
                h: "medium".to_string(),
                i: HashMap::new(),
                j: 1.23,
                k: Some(99),
                l: vec![1.1, 2.2, 3.3],
                m: false,
                n: vec!["one".to_string(), "two".to_string()],
                o: HashMap::new(),
                p: "p".to_string(),
                q: Some(vec![7, 8, 9]),
                r: vec![100, 200],
                s: HashMap::new(),
                t: 9.81,
            };
            let _ = black_box(map_medium_to_v2_reference(&m));
        })
    });

    c.bench_function("map_large_to_v2_reference", |b| {
        b.iter(|| {
            let l = Large {
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
            };
            let _ = black_box(map_large_to_v2_reference(&l));
        })
    });
}

criterion_group!(benches, benchmark_mapping);
criterion_main!(benches);
