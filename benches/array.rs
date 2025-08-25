use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};

fn make_array<const N: usize>() -> [u8; N] {
    let mut arr = [0u8; N];
    for i in 0..N {
        // Non-trivial but cheap; prevents folding everything to a constant.
        arr[i] = ((i as u32 * 31 + 7) % 251) as u8;
    }
    arr
}

fn fill_array<const N: usize>(dst: &mut [u8; N]) {
    for i in 0..N {
        dst[i] = ((i as u32 * 31 + 7) % 251) as u8;
    }
}

fn consume_by_value<const N: usize>(arr: [u8; N]) -> u64 {
    let mut s: u64 = 0;
    for &b in arr.iter() {
        s = s.wrapping_add(b as u64);
    }
    black_box(s)
}

fn consume_by_ref<const N: usize>(arr: &[u8; N]) -> u64 {
    let mut s: u64 = 0;
    for &b in arr.iter() {
        s = s.wrapping_add(b as u64);
    }
    black_box(s)
}

fn bench_return_by_value<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("return_by_value_[u8;{}]", N));
    g.throughput(Throughput::Bytes(N as u64));
    g.bench_function(BenchmarkId::from_parameter(N), |b| {
        b.iter(|| {
            let arr = make_array::<N>();
            consume_by_value::<N>(arr)
        })
    });
    g.finish();
}

fn bench_out_param<const N: usize>(c: &mut Criterion) {
    let mut g = c.benchmark_group(format!("out_param_[u8;{}]", N));
    g.throughput(Throughput::Bytes(N as u64));
    g.bench_function(BenchmarkId::from_parameter(N), |b| {
        b.iter(|| {
            let mut arr = [0u8; N];
            fill_array::<N>(&mut arr);
            consume_by_ref::<N>(&arr)
        })
    });
    g.finish();
}

fn benches(c: &mut Criterion) {
    // launch them one by one for each size
    bench_return_by_value::<10>(c);
    bench_out_param::<10>(c);

    bench_return_by_value::<100>(c);
    bench_out_param::<100>(c);

    bench_return_by_value::<1000>(c);
    bench_out_param::<1000>(c);
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
