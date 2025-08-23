use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use smallvec::{Array, SmallVec};
use tinyvec::TinyVec;

type SmallInline = [u32; 128];

#[inline(never)]
fn fill_vec_in_place<const N: usize>(v: &mut Vec<u32>) {
    v.clear(); // keep capacity, no alloc
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn fill_smallvec_in_place<const N: usize>(v: &mut SmallVec<SmallInline>) {
    v.clear(); // keeps inline/heap mode & capacity
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

#[inline(never)]
fn fill_tinyvec_in_place<const N: usize>(v: &mut TinyVec<[u32; 100]>) {
    v.clear(); // keeps inline/heap mode & capacity
    for i in 0..N as u32 {
        v.push(black_box(i));
    }
    black_box(v.as_ptr());
    black_box(v.len());
}

fn bench_fill_for<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("fill_N={}", N));
    g.throughput(Throughput::Elements(N as u64));

    // --- Preallocate once OUTSIDE the measured closure ---
    // Vec: one heap alloc here (not in the timing)
    let mut vec_buf: Vec<u32> = Vec::with_capacity(N);

    // SmallVec: inline if N<=128, else heap once here
    let inline_cap = <SmallInline as smallvec::Array>::size();
    let mut small_buf: SmallVec<SmallInline> = if N > inline_cap {
        SmallVec::with_capacity(N)
    } else {
        SmallVec::new()
    };

    // TinyVec: inline if N<=100, else heap once here
    let mut tiny_buf: TinyVec<[u32; 100]> = if N > 100 {
        TinyVec::with_capacity(N)
    } else {
        TinyVec::new()
    };

    // --- Measure only the fill (no allocations inside b.iter) ---
    g.bench_function("Vec fill (noalloc)", |b| {
        b.iter(|| fill_vec_in_place::<N>(&mut vec_buf))
    });
    g.bench_function("SmallVec fill (noalloc)", |b| {
        b.iter(|| fill_smallvec_in_place::<N>(&mut small_buf))
    });
    g.bench_function("TinyVec fill (noalloc)", |b| {
        b.iter(|| fill_tinyvec_in_place::<N>(&mut tiny_buf))
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_fill_for::<10>(c);
    bench_fill_for::<100>(c);
    bench_fill_for::<1_000>(c);
    bench_fill_for::<10_000>(c);
    bench_fill_for::<1_000_000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
