use criterion::{Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;

#[derive(Clone)]
struct Small {
    a: u32,
    b: String,
    c: f64,
}

#[derive(Clone)]
struct SmallV2 {
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
struct MediumV2 {
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

#[derive(Clone)]
struct LargeV2 {
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

// --- MAPPING FUNCTIONS ---

// Move
fn map_small_to_v2_move(input: Small) -> SmallV2 {
    SmallV2 {
        a: input.a + 1,
        b: input.b,
        c: input.c + 1.0,
    }
}

fn map_medium_to_v2_move(input: Medium) -> MediumV2 {
    MediumV2 {
        a: input.a + 1,
        b: input.b,
        c: input.c + 1.0,
        d: input.d,
        e: input.e,
        f: !input.f,
        g: input.g.map(|v| v + 1),
        h: input.h,
        i: input.i + 1.0,
        j: input.j,
    }
}

fn map_large_to_v2_move(input: Large) -> LargeV2 {
    LargeV2 {
        a: input.a + 1,
        b: input.b + 1,
        c: input.c + 1,
        d: input.d + 1,
        e: input.e,
        f: input.f,
        g: input.g,
        h: input.h,
        i: input.i + 1.0,
        j: input.j + 1.0,
        k: !input.k,
        l: input.l,
        m: input.m.map(|v| v + 1),
        n: input.n,
        o: input.o,
        p: input.p,
        q: input.q + 1,
        r: input.r + 1,
        s: input.s + 1,
        t: input.t + 1,
        u: input.u + 1,
        v: input.v + 1,
        w: input.w + 1,
        x: input.x + 1,
        y: input.y + 1,
        z: input.z + 1,
        aa: input.aa + 1,
        ab: input.ab + 1,
        ac: input.ac + 1,
        ad: input.ad + 1,
        ae: input.ae + 1,
        af: input.af + 1,
    }
}

// Clone
fn map_small_to_v2_clone(input: Small) -> SmallV2 {
    SmallV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
    }
}

fn map_medium_to_v2_clone(input: Medium) -> MediumV2 {
    MediumV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
        d: input.d.clone(),
        e: input.e.clone(),
        f: !input.f,
        g: input.g.map(|v| v + 1),
        h: input.h.clone(),
        i: input.i + 1.0,
        j: input.j.clone(),
    }
}

fn map_large_to_v2_clone(input: Large) -> LargeV2 {
    LargeV2 {
        a: input.a + 1,
        b: input.b + 1,
        c: input.c + 1,
        d: input.d + 1,
        e: input.e.clone(),
        f: input.f.clone(),
        g: input.g.clone(),
        h: input.h.clone(),
        i: input.i + 1.0,
        j: input.j + 1.0,
        k: !input.k,
        l: input.l.clone(),
        m: input.m.map(|v| v + 1),
        n: input.n.clone(),
        o: input.o.clone(),
        p: input.p.clone(),
        q: input.q + 1,
        r: input.r + 1,
        s: input.s + 1,
        t: input.t + 1,
        u: input.u + 1,
        v: input.v + 1,
        w: input.w + 1,
        x: input.x + 1,
        y: input.y + 1,
        z: input.z + 1,
        aa: input.aa + 1,
        ab: input.ab + 1,
        ac: input.ac + 1,
        ad: input.ad + 1,
        ae: input.ae + 1,
        af: input.af + 1,
    }
}

// Reference
fn map_small_to_v2_reference(input: &Small) -> SmallV2 {
    SmallV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
    }
}

fn map_medium_to_v2_reference(input: &Medium) -> MediumV2 {
    MediumV2 {
        a: input.a + 1,
        b: input.b.clone(),
        c: input.c + 1.0,
        d: input.d.clone(),
        e: input.e.clone(),
        f: !input.f,
        g: input.g.map(|v| v + 1),
        h: input.h.clone(),
        i: input.i + 1.0,
        j: input.j.clone(),
    }
}

fn map_large_to_v2_reference(input: &Large) -> LargeV2 {
    LargeV2 {
        a: input.a + 1,
        b: input.b + 1,
        c: input.c + 1,
        d: input.d + 1,
        e: input.e.clone(),
        f: input.f.clone(),
        g: input.g.clone(),
        h: input.h.clone(),
        i: input.i + 1.0,
        j: input.j + 1.0,
        k: !input.k,
        l: input.l.clone(),
        m: input.m.map(|v| v + 1),
        n: input.n.clone(),
        o: input.o.clone(),
        p: input.p.clone(),
        q: input.q + 1,
        r: input.r + 1,
        s: input.s + 1,
        t: input.t + 1,
        u: input.u + 1,
        v: input.v + 1,
        w: input.w + 1,
        x: input.x + 1,
        y: input.y + 1,
        z: input.z + 1,
        aa: input.aa + 1,
        ab: input.ab + 1,
        ac: input.ac + 1,
        ad: input.ad + 1,
        ae: input.ae + 1,
        af: input.af + 1,
    }
}

// --- BENCHMARKS ---

fn benchmark_mapping(c: &mut Criterion) {
    // Benchmark for move
    c.bench_function("map_small_to_v2_move", |b| {
        b.iter(|| {
            let s = Small {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
            };
            let _ = black_box(map_small_to_v2_move(s));
        })
    });

    c.bench_function("map_medium_to_v2_move", |b| {
        b.iter(|| {
            let m = Medium {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: "Hello".to_string(),
                i: 3.14,
                j: vec!["a".to_string(), "b".to_string()],
            };
            let _ = black_box(map_medium_to_v2_move(m));
        })
    });

    c.bench_function("map_large_to_v2_move", |b| {
        b.iter(|| {
            let l = Large {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: [0; 1024],
                f: [0; 512],
                g: vec!["Hello".to_string()],
                h: HashMap::new(),
                i: 3.14,
                j: 3.14,
                k: true,
                l: vec![1],
                m: Some(42),
                n: "Hello".to_string(),
                o: "World".to_string(),
                p: "Rust".to_string(),
                q: 1,
                r: 2,
                s: 3,
                t: 4,
                u: 5,
                v: 6,
                w: 7,
                x: 8,
                y: 9,
                z: 10,
                aa: 11,
                ab: 12,
                ac: 13,
                ad: 14,
                ae: 15,
                af: 16,
            };
            let _ = black_box(map_large_to_v2_move(l));
        })
    });

    // Benchmark for clone
    c.bench_function("map_small_to_v2_clone", |b| {
        b.iter(|| {
            let s = Small {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
            };
            let _ = black_box(map_small_to_v2_clone(s));
        })
    });

    c.bench_function("map_medium_to_v2_clone", |b| {
        b.iter(|| {
            let m = Medium {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: "Hello".to_string(),
                i: 3.14,
                j: vec!["a".to_string(), "b".to_string()],
            };
            let _ = black_box(map_medium_to_v2_clone(m));
        })
    });

    c.bench_function("map_large_to_v2_clone", |b| {
        b.iter(|| {
            let l = Large {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: [0; 1024],
                f: [0; 512],
                g: vec!["Hello".to_string()],
                h: HashMap::new(),
                i: 3.14,
                j: 3.14,
                k: true,
                l: vec![1],
                m: Some(42),
                n: "Hello".to_string(),
                o: "World".to_string(),
                p: "Rust".to_string(),
                q: 1,
                r: 2,
                s: 3,
                t: 4,
                u: 5,
                v: 6,
                w: 7,
                x: 8,
                y: 9,
                z: 10,
                aa: 11,
                ab: 12,
                ac: 13,
                ad: 14,
                ae: 15,
                af: 16,
            };
            let _ = black_box(map_large_to_v2_clone(l));
        })
    });

    // Benchmark for reference
    c.bench_function("map_small_to_v2_reference", |b| {
        b.iter(|| {
            let s = Small {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
            };
            let _ = black_box(map_small_to_v2_reference(&s));
        })
    });

    c.bench_function("map_medium_to_v2_reference", |b| {
        b.iter(|| {
            let m = Medium {
                a: 1,
                b: "Hello".to_string(),
                c: 3.14,
                d: vec![1, 2, 3],
                e: HashMap::new(),
                f: true,
                g: Some(42),
                h: "Hello".to_string(),
                i: 3.14,
                j: vec!["a".to_string(), "b".to_string()],
            };
            let _ = black_box(map_medium_to_v2_reference(&m));
        })
    });

    c.bench_function("map_large_to_v2_reference", |b| {
        b.iter(|| {
            let l = Large {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: [0; 1024],
                f: [0; 512],
                g: vec!["Hello".to_string()],
                h: HashMap::new(),
                i: 3.14,
                j: 3.14,
                k: true,
                l: vec![1],
                m: Some(42),
                n: "Hello".to_string(),
                o: "World".to_string(),
                p: "Rust".to_string(),
                q: 1,
                r: 2,
                s: 3,
                t: 4,
                u: 5,
                v: 6,
                w: 7,
                x: 8,
                y: 9,
                z: 10,
                aa: 11,
                ab: 12,
                ac: 13,
                ad: 14,
                ae: 15,
                af: 16,
            };
            let _ = black_box(map_large_to_v2_reference(&l));
        })
    });
}

criterion_group!(benches, benchmark_mapping);
criterion_main!(benches);
