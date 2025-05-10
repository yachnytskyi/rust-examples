use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::prelude::*;
use std::collections::HashMap;

fn bench_vec_and_map_search(c: &mut Criterion) {
    // Sizes to test
    let sizes = [10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];

    for &size in &sizes {
        // Create a new unsorted Vec and HashMap for this size
        let mut rng = rand::rng();

        let vec: Vec<u32> = (0..size).map(|_| rng.random_range(0..size as u32)).collect();
        let mut map: HashMap<u32, ()> = HashMap::with_capacity(size);
        for &v in &vec {
            map.insert(v, ());
        }

        // Make a sorted clone for pre-sorted binary search
        let mut sorted_vec = vec.clone();
        sorted_vec.sort_unstable();

        // Value to search for — a number not present (for worst case)
        let search_value = size as u32 + 1;

        // Benchmark: linear search in Vec
        let vec_bench_id = format!("vec_linear_search_{} elements", size);
        c.bench_function(&vec_bench_id, |b| {
            b.iter(|| {
                black_box(vec.iter().find(|&&x| x == search_value));
            })
        });

        // Benchmark: HashMap lookup
        let map_bench_id = format!("map_search_{} elements", size);
        c.bench_function(&map_bench_id, |b| {
            b.iter(|| {
                black_box(map.get(&search_value));
            })
        });

        // Benchmark: binary search in pre-sorted Vec
        let bin_search_id = format!("vec_binary_search_presorted_{} elements", size);
        c.bench_function(&bin_search_id, |b| {
            b.iter(|| {
                black_box(sorted_vec.binary_search(&search_value).ok());
            })
        });

        // Benchmark: sort and then binary search within benchmark run
        let sort_and_bin_search_id = format!("vec_sort_and_binary_search_{} elements", size);
        c.bench_function(&sort_and_bin_search_id, |b| {
            b.iter(|| {
                let mut vec_copy = vec.clone();
                vec_copy.sort_unstable();
                black_box(vec_copy.binary_search(&search_value).ok());
            })
        });
    }
}

criterion_group!(benches, bench_vec_and_map_search);
criterion_main!(benches);
