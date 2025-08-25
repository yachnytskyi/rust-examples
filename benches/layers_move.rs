use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use std::collections::HashMap;

// ---------------------------------------
// Domain types
// ---------------------------------------

#[derive(Debug)]
struct Payload {
    id: u64,
    name: String,
    data: Vec<u8>,
    tags: Vec<String>,
    props: HashMap<String, u32>,
}

#[derive(Debug)]
struct PayloadBig {
    id: u64,
    name: String,
    email: String,
    address: String,
    notes: String,
    phone: String,
    company: String,
    tags: Vec<String>,
    data: Vec<u8>,
    meta: HashMap<String, u64>,
    props: HashMap<String, u32>,
    numbers: Vec<u64>,
    flags: Vec<bool>,
    codes: Vec<String>,
    desc1: String,
    desc2: String,
    desc3: String,
    desc4: String,
    desc5: String,
    vec1: Vec<u8>,
    vec2: Vec<u8>,
    vec3: Vec<u8>,
    vec4: Vec<u8>,
    vec5: Vec<u8>,
    opt1: Option<String>,
    opt2: Option<String>,
    opt3: Option<String>,
    opt4: Option<String>,
    opt5: Option<String>,
    opt6: Option<u64>,
}

// ---------------------------------------
// Builders (no clone(); each field is built independently)
// ---------------------------------------

fn build_payload(bytes: usize, ntags: usize, keyspace: usize) -> Payload {
    let mut data = Vec::with_capacity(bytes);
    let seed = black_box(123u8);
    for i in 0..bytes {
        data.push(seed.wrapping_add((i as u8).wrapping_mul(31)));
    }

    let mut tags = Vec::with_capacity(ntags);
    for i in 0..ntags {
        // unique strings; not cloning an existing String
        tags.push(format!("tag_{i:04}"));
    }

    let mut props = HashMap::with_capacity(keyspace);
    for i in 0..keyspace {
        props.insert(format!("k{i:05}"), (i as u32).wrapping_mul(7));
    }

    Payload {
        id: 42,
        name: format!("example_{bytes}_{ntags}_{keyspace}"),
        data,
        tags,
        props,
    }
}

fn build_payload_big(bytes: usize, ntags: usize, keyspace: usize) -> PayloadBig {
    // big byte blob
    let mut data = Vec::with_capacity(bytes);
    for i in 0..bytes {
        data.push((i % 251) as u8);
    }

    // tags
    let mut tags = Vec::with_capacity(ntags);
    for i in 0..ntags {
        tags.push(format!("tag_{i}"));
    }

    // maps
    let mut meta = HashMap::with_capacity(keyspace);
    let mut props = HashMap::with_capacity(keyspace);
    for i in 0..keyspace {
        meta.insert(format!("meta{i}"), i as u64);
        props.insert(format!("prop{i}"), i as u32);
    }

    // many independent owned Strings (no clones of one base)
    PayloadBig {
        id: 99,
        name: format!("name_{bytes}_{ntags}_{keyspace}"),
        email: "user@example.com".to_string(),
        address: "Street 1, City".to_string(),
        notes: "notes here".to_string(),
        phone: "+123456789".to_string(),
        company: "ACME".to_string(),
        tags,
        data,
        meta,
        props,
        numbers: vec![1, 2, 3, 4, 5],
        flags: vec![true, false, true],
        codes: vec!["A".into(), "B".into(), "C".into()],
        desc1: "d1".to_string(),
        desc2: "d2".to_string(),
        desc3: "d3".to_string(),
        desc4: "d4".to_string(),
        desc5: "d5".to_string(),
        vec1: vec![1, 2, 3],
        vec2: vec![4, 5, 6],
        vec3: vec![7, 8, 9],
        vec4: vec![10, 11, 12],
        vec5: vec![13, 14, 15],
        opt1: Some("opt1".into()),
        opt2: Some("opt2".into()),
        opt3: Some("opt3".into()),
        opt4: Some("opt4".into()),
        opt5: Some("opt5".into()),
        opt6: Some(123456),
    }
}

// ---------------------------------------
// Consumers (force use so optimizer can't drop work)
// ---------------------------------------

fn consume_payload(p: Payload) -> usize {
    let mut acc = p.id as usize ^ p.name.len();
    acc ^= p.data.len();
    acc ^= p.tags.len();
    acc ^= p.props.len();
    black_box(acc)
}

fn consume_payload_big(p: PayloadBig) -> usize {
    let mut acc = p.id as usize ^ p.name.len();
    acc ^= p.data.len();
    acc ^= p.tags.len();
    acc ^= p.props.len();
    acc ^= p.meta.len();
    acc ^= p.codes.len();
    black_box(acc)
}

// ---------------------------------------
// Layers (ownership just forwarded; no clones)
// ---------------------------------------

fn one_layer_p(p: Payload) -> Payload {
    p
}
fn repo_p(p: Payload) -> Payload {
    p
}
fn usecase_p(p: Payload) -> Payload {
    repo_p(p)
}
fn controller_p(p: Payload) -> Payload {
    usecase_p(p)
}

fn one_layer_b(p: PayloadBig) -> PayloadBig {
    p
}
fn repo_b(p: PayloadBig) -> PayloadBig {
    p
}
fn usecase_b(p: PayloadBig) -> PayloadBig {
    repo_b(p)
}
fn controller_b(p: PayloadBig) -> PayloadBig {
    usecase_b(p)
}

// ---------------------------------------
// Benches
// ---------------------------------------

fn bench_case(c: &mut Criterion, bytes: usize, ntags: usize, keyspace: usize) {
    let label = format!("bytes={bytes},tags={ntags},keys={keyspace}");
    let mut g = c.benchmark_group(format!("layer_moves/{label}"));

    // Heuristic throughput label: primary owned bytes (just for display)
    let approx_bytes = bytes + ntags * 8 + keyspace * 8;
    g.throughput(Throughput::Bytes(approx_bytes as u64));

    // ---- Payload (build inside iter) ----
    g.bench_function(BenchmarkId::new("Payload_1_layer", &label), |b| {
        b.iter(|| {
            let p = build_payload(bytes, ntags, keyspace);
            let p = one_layer_p(p);
            consume_payload(p)
        })
    });

    g.bench_function(BenchmarkId::new("Payload_3_layers", &label), |b| {
        b.iter(|| {
            let p = build_payload(bytes, ntags, keyspace);
            let p = controller_p(p);
            consume_payload(p)
        })
    });

    // ---- PayloadBig (build inside iter) ----
    g.bench_function(BenchmarkId::new("PayloadBig_1_layer", &label), |b| {
        b.iter(|| {
            let p = build_payload_big(bytes, ntags, keyspace);
            let p = one_layer_b(p);
            consume_payload_big(p)
        })
    });

    g.bench_function(BenchmarkId::new("PayloadBig_3_layers", &label), |b| {
        b.iter(|| {
            let p = build_payload_big(bytes, ntags, keyspace);
            let p = controller_b(p);
            consume_payload_big(p)
        })
    });

    // ---- Forward-only variants (isolate the hops) ----
    g.bench_function(
        BenchmarkId::new("Payload_forward_only_1_layer", &label),
        |b| {
            b.iter_batched(
                || build_payload(bytes, ntags, keyspace), // setup (not timed)
                |p| consume_payload(one_layer_p(p)),      // measured: hop + consume
                BatchSize::SmallInput,
            )
        },
    );

    g.bench_function(
        BenchmarkId::new("Payload_forward_only_3_layers", &label),
        |b| {
            b.iter_batched(
                || build_payload(bytes, ntags, keyspace),
                |p| consume_payload(controller_p(p)),
                BatchSize::SmallInput,
            )
        },
    );

    g.bench_function(
        BenchmarkId::new("PayloadBig_forward_only_1_layer", &label),
        |b| {
            b.iter_batched(
                || build_payload_big(bytes, ntags, keyspace),
                |p| consume_payload_big(one_layer_b(p)),
                BatchSize::SmallInput,
            )
        },
    );

    g.bench_function(
        BenchmarkId::new("PayloadBig_forward_only_3_layers", &label),
        |b| {
            b.iter_batched(
                || build_payload_big(bytes, ntags, keyspace),
                |p| consume_payload_big(controller_b(p)),
                BatchSize::SmallInput,
            )
        },
    );

    g.finish();
}

fn benches(c: &mut Criterion) {
    bench_case(c, 1024, 8, 8); // small
    bench_case(c, 8192, 16, 16); // medium
    bench_case(c, 65536, 64, 64); // larger
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
