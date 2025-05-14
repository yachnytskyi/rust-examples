use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[derive(Clone)]
struct StringWrapper {
    value: String,
}

#[derive(Clone)]
struct StrWrapper<'a> {
    value: &'a str,
}

fn no_modification_string() -> StringWrapper {
    StringWrapper {
        value: String::from("Hello, world!"),
    }
}

fn no_modification_str<'a>() -> StrWrapper<'a> {
    let s: &'a str = "Hello, world!";
    StrWrapper { value: s }
}

fn modify_once_string() -> StringWrapper {
    let mut s = StringWrapper {
        value: String::from("Hello"),
    };
    s.value.push_str(", world!"); // Modify once
    s
}

fn modify_once_str<'a>() -> StrWrapper<'a> {
    let s: &'a str = "Hello";
    let new_str = format!("{}, world!", s); // Create a new string, because `str` is immutable
    let new_str_ref: &'a str = Box::leak(new_str.into_boxed_str());
    StrWrapper { value: new_str_ref }
}

fn modify_four_times_string() -> StringWrapper {
    let mut s = StringWrapper {
        value: String::from("Hello"),
    };
    for _ in 0..4 {
        s.value.push_str(" world");
    }
    s
}

fn modify_four_times_str<'a>() -> StrWrapper<'a> {
    let s: &'a str = "Hello";
    let mut new_str = s.to_string();
    for _ in 0..4 {
        new_str.push_str(" world");
    }
    let new_str_ref: &'a str = Box::leak(new_str.into_boxed_str());
    StrWrapper { value: new_str_ref }
}

fn modify_ten_times_string() -> StringWrapper {
    let mut s = StringWrapper {
        value: String::from("Hello"),
    };
    for _ in 0..10 {
        s.value.push_str(" world");
    }
    s
}

fn modify_ten_times_str<'a>() -> StrWrapper<'a> {
    let s: &'a str = "Hello";
    let mut new_str = s.to_string();
    for _ in 0..10 {
        new_str.push_str(" world");
    }
    let new_str_ref: &'a str = Box::leak(new_str.into_boxed_str());
    StrWrapper { value: new_str_ref }
}

fn modify_multiple_times_string() -> StringWrapper {
    let mut s = StringWrapper {
        value: String::from("Hello"),
    };
    for _ in 0..1000 {
        s.value.push_str(" world");
    }
    s
}

fn modify_multiple_times_str<'a>() -> StrWrapper<'a> {
    let s: &'a str = "Hello";
    let mut new_str = s.to_string();
    for _ in 0..1000 {
        new_str.push_str(" world");
    }
    let new_str_ref: &'a str = Box::leak(new_str.into_boxed_str());
    StrWrapper { value: new_str_ref }
}

fn benchmark_string(c: &mut Criterion) {
    // Benchmarking no modification for String
    c.bench_function("no_modification_string", |b| {
        b.iter(|| black_box(no_modification_string()))
    });

    // Benchmarking modifying once for String
    c.bench_function("modify_once_string", |b| {
        b.iter(|| black_box(modify_once_string()))
    });

    // Benchmarking modifying 4 times for String
    c.bench_function("modify_four_times_string", |b| {
        b.iter(|| black_box(modify_four_times_string()))
    });

    // Benchmarking modifying 10 times for String
    c.bench_function("modify_ten_times_string", |b| {
        b.iter(|| black_box(modify_ten_times_string()))
    });

    // Benchmarking modifying multiple times for String
    c.bench_function("modify_multiple_times_string", |b| {
        b.iter(|| black_box(modify_multiple_times_string()))
    });
}

fn benchmark_str(c: &mut Criterion) {
    // Benchmarking no modification for &str
    c.bench_function("no_modification_str", |b| {
        b.iter(|| black_box(no_modification_str()))
    });

    // Benchmarking modifying once for &str
    c.bench_function("modify_once_str", |b| {
        b.iter(|| black_box(modify_once_str()))
    });

    // Benchmarking modifying 4 times for &str
    c.bench_function("modify_four_times_str", |b| {
        b.iter(|| black_box(modify_four_times_str()))
    });

    // Benchmarking modifying 10 times for &str
    c.bench_function("modify_ten_times_str", |b| {
        b.iter(|| black_box(modify_ten_times_str()))
    });

    // Benchmarking modifying multiple times for &str
    c.bench_function("modify_multiple_times_str", |b| {
        b.iter(|| black_box(modify_multiple_times_str()))
    });
}

criterion_group!(benches, benchmark_string, benchmark_str);
criterion_main!(benches);
