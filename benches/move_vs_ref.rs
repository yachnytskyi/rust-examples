use criterion::{Criterion, black_box, criterion_group, criterion_main};

#[derive(Clone)]
struct Small {
    a: u32,
    b: u32,
}

#[derive(Clone)]
struct Medium {
    a: u32,
    b: u32,
    c: u64,
    d: u64,
    e: [u8; 256],
    f: [u32; 64],
    g: u64,
}

#[derive(Clone)]
struct Large {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: [u8; 2048],
    f: [u32; 1024],
    g: [u64; 128],
    h: [u16; 512],
    i: u64,
}

// Functions that return by move
fn return_small_move() -> Small {
    Small { a: 1, b: 2 }
}

fn return_medium_move() -> Medium {
    Medium {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: [0; 256],
        f: [0; 64],
        g: 5,
    }
}

fn return_large_move() -> Large {
    Large {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: [0; 2048],
        f: [0; 1024],
        g: [0; 128],
        h: [0; 512],
        i: 5,
    }
}

// Functions that return by reference
fn return_small_ref<'a>(val: &'a Small) -> &'a Small {
    val
}

fn return_medium_ref<'a>(val: &'a Medium) -> &'a Medium {
    val
}

fn return_large_ref<'a>(val: &'a Large) -> &'a Large {
    val
}

// Functions that return by mutable reference
fn return_small_mut_ref<'a>(val: &'a mut Small) -> &'a mut Small {
    val
}

fn return_medium_mut_ref<'a>(val: &'a mut Medium) -> &'a mut Medium {
    val
}

fn return_large_mut_ref<'a>(val: &'a mut Large) -> &'a mut Large {
    val
}

// Functions that return by clone
fn return_small_clone() -> Small {
    Small { a: 1, b: 2 }.clone()
}

fn return_medium_clone() -> Medium {
    Medium {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: [0; 256],
        f: [0; 64],
        g: 5,
    }
    .clone()
}

fn return_large_clone() -> Large {
    Large {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: [0; 2048],
        f: [0; 1024],
        g: [0; 128],
        h: [0; 512],
        i: 5,
    }
    .clone()
}

// Benchmark functions for cloning
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

// Benchmark functions for moves
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

// Benchmark functions for references
fn benchmark_refs(c: &mut Criterion) {
    c.bench_function("return_small_ref", |b| {
        b.iter(|| {
            let s = Small { a: 1, b: 2 };
            let _ = black_box(return_small_ref(&s));
        })
    });

    c.bench_function("return_medium_ref", |b| {
        b.iter(|| {
            let m = return_medium_move();
            let _ = black_box(return_medium_ref(&m));
        })
    });

    c.bench_function("return_large_ref", |b| {
        b.iter(|| {
            let l = return_large_move();
            let _ = black_box(return_large_ref(&l));
        })
    });
}

// Benchmark functions for mutable references
fn benchmark_mut_refs(c: &mut Criterion) {
    c.bench_function("return_small_mut_ref", |b| {
        b.iter(|| {
            let mut s = Small { a: 1, b: 2 };
            let _ = black_box(return_small_mut_ref(&mut s));
        })
    });

    c.bench_function("return_medium_mut_ref", |b| {
        b.iter(|| {
            let mut m = return_medium_move();
            let _ = black_box(return_medium_mut_ref(&mut m));
        })
    });

    c.bench_function("return_large_mut_ref", |b| {
        b.iter(|| {
            let mut l = return_large_move();
            let _ = black_box(return_large_mut_ref(&mut l));
        })
    });
}

criterion_group!(
    benches,
    benchmark_clones,
    benchmark_moves,
    benchmark_refs,
    benchmark_mut_refs,
);
criterion_main!(benches);
