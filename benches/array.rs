use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::mem::MaybeUninit;
use std::ptr;

#[inline(never)]
fn id_return_noinline<const N: usize>(a: [u8; N]) -> [u8; N] {
    a
}

#[inline(never)]
fn manual_memcpy_noinline<const N: usize>(src: &[u8; N], dst: &mut [u8; N]) {
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), N);
    }
}

#[inline(always)]
fn build_return_inline<const N: usize>(seed: u8) -> [u8; N] {
    let mut out = [0u8; N];
    for i in 0..N {
        out[i] = seed.wrapping_add(i as u8);
    }
    out
}

#[inline(always)]
fn build_outparam_inline<const N: usize>(seed: u8, out: &mut [u8; N]) {
    for i in 0..N {
        out[i] = seed.wrapping_add(i as u8);
    }
}

#[inline(never)]
fn consume_sum<const N: usize>(a: &[u8; N]) -> u64 {
    let mut s = 0u64;
    for &b in a {
        s = s.wrapping_add(b as u64);
    }
    black_box(s)
}

fn make_src<const N: usize>() -> [u8; N] {
    let mut a = [0u8; N];
    let seed = black_box(123u8);
    for i in 0..N {
        a[i] = seed.wrapping_add(i as u8);
    }
    black_box(a)
}

fn bench_size<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("return_overhead_[u8;{}]", N));
    g.throughput(Throughput::Bytes(N as u64));

    // Baseline: manual memcpy (no return-by-value involved)
    g.bench_function(BenchmarkId::new("manual_memcpy_noinline", N), |b| {
        b.iter(|| {
            let src = make_src::<N>();
            let mut dst = MaybeUninit::<[u8; N]>::uninit();
            let dst_ref: &mut [u8; N] = unsafe { &mut *dst.as_mut_ptr() };
            manual_memcpy_noinline::<N>(&src, dst_ref);
            let borrow: &[u8; N] = &*dst_ref;
            consume_sum::<N>(black_box(borrow))
        })
    });

    // Return-by-value identity: forces sret + one copy of existing bytes
    g.bench_function(BenchmarkId::new("id_return_noinline", N), |b| {
        b.iter(|| {
            let src = make_src::<N>();
            // Pass by value so the callee has its own object; returning causes one copy into sret.
            let out = id_return_noinline::<N>(src);
            consume_sum::<N>(black_box(&out))
        })
    });

    // Best-case build: return-by-value, constructed in caller’s sret (no extra copy)
    g.bench_function(BenchmarkId::new("build_return_inline", N), |b| {
        b.iter(|| {
            let out = build_return_inline::<N>(black_box(123));
            consume_sum::<N>(black_box(&out))
        })
    });

    // Best-case build: outparam (same as above, just explicit)
    g.bench_function(BenchmarkId::new("build_outparam_inline", N), |b| {
        b.iter(|| {
            let mut out = MaybeUninit::<[u8; N]>::uninit();
            let out_ref: &mut [u8; N] = unsafe { &mut *out.as_mut_ptr() };
            build_outparam_inline::<N>(black_box(123), out_ref);
            let borrow: &[u8; N] = &*out_ref;
            consume_sum::<N>(black_box(borrow))
        })
    });

    g.finish();
}

fn benches(c: &mut Criterion) {
    // include N=0 to show the near-constant “pure return” overhead baseline
    bench_size::<0>(c);
    bench_size::<10>(c);
    bench_size::<32>(c);
    bench_size::<64>(c);
    bench_size::<100>(c);
    bench_size::<256>(c);
    bench_size::<512>(c);
    bench_size::<1000>(c);
    bench_size::<2048>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
