use criterion::{BatchSize, Criterion, Throughput, black_box, criterion_group, criterion_main};
use smallvec::SmallVec;
use tinyvec::TinyVec;

// Tune to taste:
const REPS: usize = 100_000; // how many tiny vectors we create & drop per iteration
const K: usize = 8; // inline size / typical tiny size (keep <= K to avoid heap)

// ---------- helpers ----------
fn do_work_on(v: &[u32]) -> u64 {
    // Minimal use to prevent optimization-away.
    v.iter().fold(0u64, |acc, &x| acc.wrapping_add(x as u64))
}

// Fixed-size (always exactly K)
fn many_tiny_vec_fixed_k() -> u64 {
    let mut acc = 0u64;
    for _ in 0..REPS {
        // Alloc per tiny Vec when created fresh
        let mut v: Vec<u32> = Vec::new();
        for i in 0..(K as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

fn many_tiny_vec_fixed_k_reserve() -> u64 {
    let mut acc = 0u64;
    for _ in 0..REPS {
        // Still allocates each time (but avoids growth steps)
        let mut v: Vec<u32> = Vec::with_capacity(K);
        for i in 0..(K as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

fn many_tiny_smallvec_fixed_k() -> u64 {
    let mut acc = 0u64;
    for _ in 0..REPS {
        // Inline storage → no heap alloc
        let mut v: SmallVec<[u32; K]> = SmallVec::new();
        for i in 0..(K as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

fn many_tiny_tinyvec_fixed_k() -> u64 {
    let mut acc = 0u64;
    for _ in 0..REPS {
        // Inline storage → no heap alloc
        let mut v: TinyVec<[u32; K]> = TinyVec::new();
        for i in 0..(K as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

// Varied-size (0..=K) via tiny LCG — closer to “sometimes small”
#[inline(always)]
fn lcg_step(x: &mut u64) -> usize {
    *x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
    ((*x >> 32) as usize) % (K + 1)
}

fn many_tiny_vec_varied() -> u64 {
    let mut acc = 0u64;
    let mut rnd = 0xDEADBEEFCAFEBABEu64;
    for _ in 0..REPS {
        let sz = lcg_step(&mut rnd);
        let mut v: Vec<u32> = Vec::new(); // alloc per tiny Vec
        for i in 0..(sz as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

fn many_tiny_vec_varied_reserve() -> u64 {
    let mut acc = 0u64;
    let mut rnd = 0xFACEFEED12345678u64;
    for _ in 0..REPS {
        let sz = lcg_step(&mut rnd);
        let mut v: Vec<u32> = Vec::with_capacity(K); // still alloc each time
        for i in 0..(sz as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

fn many_tiny_smallvec_varied() -> u64 {
    let mut acc = 0u64;
    let mut rnd = 0xA5A5A5A5A5A5A5A5u64;
    for _ in 0..REPS {
        let sz = lcg_step(&mut rnd);
        let mut v: SmallVec<[u32; K]> = SmallVec::new(); // inline for sz<=K
        for i in 0..(sz as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

fn many_tiny_tinyvec_varied() -> u64 {
    let mut acc = 0u64;
    let mut rnd = 0x0123456789ABCDEFu64;
    for _ in 0..REPS {
        let sz = lcg_step(&mut rnd);
        let mut v: TinyVec<[u32; K]> = TinyVec::new(); // inline for sz<=K
        for i in 0..(sz as u32) {
            v.push(black_box(i));
        }
        acc = acc.wrapping_add(do_work_on(&v));
    }
    acc
}

// ---------- benches ----------
fn bench_many_tiny_fixed(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("many_tiny_fixed_K={}_REPS={}", K, REPS));
    // Total pushed elements per iteration: REPS * K
    g.throughput(Throughput::Elements((REPS as u64) * (K as u64)));

    g.bench_function("Vec (alloc per tiny vec)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_vec_fixed_k()),
            BatchSize::SmallInput,
        )
    });
    g.bench_function("Vec reserve K (alloc per tiny vec)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_vec_fixed_k_reserve()),
            BatchSize::SmallInput,
        )
    });
    g.bench_function("SmallVec<[K]> (inline, noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_smallvec_fixed_k()),
            BatchSize::SmallInput,
        )
    });
    g.bench_function("TinyVec<[K]> (inline, noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_tinyvec_fixed_k()),
            BatchSize::SmallInput,
        )
    });

    g.finish();
}

fn bench_many_tiny_varied(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("many_tiny_varied_<=K K={}_REPS={}", K, REPS));
    // Expected total pushes ≈ REPS * (average of 0..=K) = REPS * (K/2)
    // Use u64 arithmetic first to avoid E0605 / trait errors.
    g.throughput(Throughput::Elements((REPS as u64) * (K as u64) / 2));

    g.bench_function("Vec (alloc per tiny vec)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_vec_varied()),
            BatchSize::SmallInput,
        )
    });
    g.bench_function("Vec reserve K (alloc per tiny vec)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_vec_varied_reserve()),
            BatchSize::SmallInput,
        )
    });
    g.bench_function("SmallVec<[K]> (inline, noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_smallvec_varied()),
            BatchSize::SmallInput,
        )
    });
    g.bench_function("TinyVec<[K]> (inline, noalloc)", |b| {
        b.iter_batched(
            || (),
            |_| black_box(many_tiny_tinyvec_varied()),
            BatchSize::SmallInput,
        )
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_many_tiny_fixed(c);
    bench_many_tiny_varied(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
