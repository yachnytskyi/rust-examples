use criterion::{Criterion, black_box, criterion_group, criterion_main};
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

fn return_small_move() -> Small {
    Small {
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
    }
}

fn return_medium_move() -> Medium {
    Medium {
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
    }
}

fn return_large_move() -> Large {
    Large {
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
    }
}

fn return_small_ref<'a>(val: &'a Small) -> &'a Small {
    val
}

fn return_medium_ref<'a>(val: &'a Medium) -> &'a Medium {
    val
}

fn return_large_ref<'a>(val: &'a Large) -> &'a Large {
    val
}

fn return_small_mut_ref<'a>(val: &'a mut Small) -> &'a mut Small {
    val
}

fn return_medium_mut_ref<'a>(val: &'a mut Medium) -> &'a mut Medium {
    val
}

fn return_large_mut_ref<'a>(val: &'a mut Large) -> &'a mut Large {
    val
}

fn return_small_clone() -> Small {
    Small {
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
    }
    .clone()
}

fn return_medium_clone() -> Medium {
    Medium {
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
    }
    .clone()
}

fn return_large_clone() -> Large {
    Large {
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
    }
    .clone()
}

fn benchmark_clones(c: &mut Criterion) {
    c.bench_function("return_small_clone", |b| {
        b.iter(|| {
            let _ = black_box(return_small_clone());
        })
    });

    c.bench_function("return_medium_clone", |b| {
        b.iter(|| {
            let _ = black_box(return_medium_clone());
        })
    });

    c.bench_function("return_large_clone", |b| {
        b.iter(|| {
            let _ = black_box(return_large_clone());
        })
    });
}

fn benchmark_moves(c: &mut Criterion) {
    c.bench_function("return_small_move", |b| {
        b.iter(|| {
            let _ = black_box(return_small_move());
        })
    });

    c.bench_function("return_medium_move", |b| {
        b.iter(|| {
            let _ = black_box(return_medium_move());
        })
    });

    c.bench_function("return_large_move", |b| {
        b.iter(|| {
            let _ = black_box(return_large_move());
        })
    });
}

fn benchmark_refs(c: &mut Criterion) {
    let s = Small {
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
    };

    let m = Medium {
        a: 0,
        b: String::new(),
        c: 0.0,
        d: Vec::new(),
        e: HashMap::new(),
        f: 0,
        g: Vec::new(),
        h: String::new(),
        i: HashMap::new(),
        j: 0.0,
        k: None,
        l: Vec::new(),
        m: false,
        n: Vec::new(),
        o: HashMap::new(),
        p: String::new(),
        q: None,
        r: Vec::new(),
        s: HashMap::new(),
        t: 0.0,
    };

    let l = Large {
        a: 0,
        b: 0.0,
        c: String::new(),
        d: Vec::new(),
        e: HashMap::new(),
        f: None,
        g: 0,
        h: Vec::new(),
        i: HashMap::new(),
        j: 0.0,
        k: 0,
        l: Vec::new(),
        m: String::new(),
        n: HashMap::new(),
        o: Vec::new(),
        p: Vec::new(),
        q: Vec::new(),
        r: HashMap::new(),
        s: Vec::new(),
        t: None,
        u: 0.0,
        v: String::new(),
        w: 0,
        x: 0.0,
        y: 0,
        z: String::new(),
        aa: Vec::new(),
        ab: 0,
        ac: HashMap::new(),
        ad: None,
        ae: Vec::new(),
        af: HashMap::new(),
    };

    c.bench_function("return_small_ref", |b| {
        b.iter(|| {
            let _ = black_box(return_small_ref(&s));
        })
    });

    c.bench_function("return_medium_ref", |b| {
        b.iter(|| {
            let _ = black_box(return_medium_ref(&m));
        })
    });

    c.bench_function("return_large_ref", |b| {
        b.iter(|| {
            let _ = black_box(return_large_ref(&l));
        })
    });
}

fn benchmark_mut_refs(c: &mut Criterion) {
    let mut s = Small {
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
    };

    let mut m = Medium {
        a: 0,
        b: String::new(),
        c: 0.0,
        d: Vec::new(),
        e: HashMap::new(),
        f: 0,
        g: Vec::new(),
        h: String::new(),
        i: HashMap::new(),
        j: 0.0,
        k: None,
        l: Vec::new(),
        m: false,
        n: Vec::new(),
        o: HashMap::new(),
        p: String::new(),
        q: None,
        r: Vec::new(),
        s: HashMap::new(),
        t: 0.0,
    };

    let mut l = Large {
        a: 0,
        b: 0.0,
        c: String::new(),
        d: Vec::new(),
        e: HashMap::new(),
        f: None,
        g: 0,
        h: Vec::new(),
        i: HashMap::new(),
        j: 0.0,
        k: 0,
        l: Vec::new(),
        m: String::new(),
        n: HashMap::new(),
        o: Vec::new(),
        p: Vec::new(),
        q: Vec::new(),
        r: HashMap::new(),
        s: Vec::new(),
        t: None,
        u: 0.0,
        v: String::new(),
        w: 0,
        x: 0.0,
        y: 0,
        z: String::new(),
        aa: Vec::new(),
        ab: 0,
        ac: HashMap::new(),
        ad: None,
        ae: Vec::new(),
        af: HashMap::new(),
    };

    c.bench_function("return_small_mut_ref", |b| {
        b.iter(|| {
            let _ = black_box(return_small_mut_ref(&mut s));
        })
    });

    c.bench_function("return_medium_mut_ref", |b| {
        b.iter(|| {
            let _ = black_box(return_medium_mut_ref(&mut m));
        })
    });

    c.bench_function("return_large_mut_ref", |b| {
        b.iter(|| {
            let _ = black_box(return_large_mut_ref(&mut l));
        })
    });
}


criterion_group!(
    benches,
    benchmark_clones,
    benchmark_moves,
    benchmark_refs,
    benchmark_mut_refs
);

criterion_main!(benches);
