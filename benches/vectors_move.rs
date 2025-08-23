use criterion::{Criterion, black_box, criterion_group, criterion_main};
use smallvec::{Array, SmallVec};
use tinyvec::TinyVec;

// ---------- Builders (generic over N) ----------
fn make_vec_n<const N: usize>() -> Vec<u32> {
    let mut v = Vec::with_capacity(N);
    for i in 0..N as u32 {
        v.push(i);
    }
    v
}

// SmallVec inline cap = 128. For N <= 128 this stays inline; for N > 128 this allocates once.
type SmallInline = [u32; 128];
fn make_smallvec_n<const N: usize>() -> SmallVec<SmallInline> {
    // smallvec 1.15: Array::size() gives inline capacity
    let inline_cap = <SmallInline as Array>::size();

    let mut v: SmallVec<SmallInline> = if N > inline_cap {
        SmallVec::with_capacity(N)
    } else {
        SmallVec::new()
    };

    for i in 0..N as u32 {
        v.push(i);
    }
    v
}

// TinyVec inline cap = 100. Same idea.
fn make_tinyvec_n<const N: usize>() -> TinyVec<[u32; 100]> {
    let mut v: TinyVec<[u32; 100]> = if N > 100 {
        TinyVec::with_capacity(N)
    } else {
        TinyVec::new()
    };
    for i in 0..N as u32 {
        v.push(i);
    }
    v
}

// ---------- Repo → UseCase (return by move) ----------
pub trait Repo<T> {
    fn fetch(&self) -> T;
}

pub struct VecRepo<const N: usize>;
pub struct SmallVecRepo<const N: usize>;
pub struct TinyVecRepo<const N: usize>;

impl<const N: usize> Repo<Vec<u32>> for VecRepo<N> {
    fn fetch(&self) -> Vec<u32> {
        make_vec_n::<N>()
    }
}
impl<const N: usize> Repo<SmallVec<SmallInline>> for SmallVecRepo<N> {
    fn fetch(&self) -> SmallVec<SmallInline> {
        make_smallvec_n::<N>()
    }
}
impl<const N: usize> Repo<TinyVec<[u32; 100]>> for TinyVecRepo<N> {
    fn fetch(&self) -> TinyVec<[u32; 100]> {
        make_tinyvec_n::<N>()
    }
}

pub struct UseCase<R> {
    repo: R,
}
impl<R> UseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<const N: usize> UseCase<VecRepo<N>> {
    pub fn run_vec(&self) -> Vec<u32> {
        self.repo.fetch()
    }
}
impl<const N: usize> UseCase<SmallVecRepo<N>> {
    pub fn run_smallvec(&self) -> SmallVec<SmallInline> {
        self.repo.fetch()
    }
}
impl<const N: usize> UseCase<TinyVecRepo<N>> {
    pub fn run_tinyvec(&self) -> TinyVec<[u32; 100]> {
        self.repo.fetch()
    }
}

// ---------- Repo → UseCase (return Box<[u32]> by move) ----------
pub trait RepoBox {
    fn fetch_box(&self) -> Box<[u32]>;
}

pub struct VecRepoBox<const N: usize>;
pub struct SmallVecRepoBox<const N: usize>;
pub struct TinyVecRepoBox<const N: usize>;

impl<const N: usize> RepoBox for VecRepoBox<N> {
    fn fetch_box(&self) -> Box<[u32]> {
        // Vec preallocated exactly: reuses buffer, no extra alloc/copy
        make_vec_n::<N>().into_boxed_slice()
    }
}
impl<const N: usize> RepoBox for SmallVecRepoBox<N> {
    fn fetch_box(&self) -> Box<[u32]> {
        // If inline, this allocates + memcpy; if already heap, buffer is reused.
        make_smallvec_n::<N>().into_vec().into_boxed_slice()
    }
}
impl<const N: usize> RepoBox for TinyVecRepoBox<N> {
    fn fetch_box(&self) -> Box<[u32]> {
        make_tinyvec_n::<N>().into_vec().into_boxed_slice()
    }
}

pub struct UseCaseBox<R: RepoBox> {
    repo: R,
}
impl<R: RepoBox> UseCaseBox<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}
impl<const N: usize> UseCaseBox<VecRepoBox<N>> {
    pub fn run_box(&self) -> Box<[u32]> {
        self.repo.fetch_box()
    }
}
impl<const N: usize> UseCaseBox<SmallVecRepoBox<N>> {
    pub fn run_box(&self) -> Box<[u32]> {
        self.repo.fetch_box()
    }
}
impl<const N: usize> UseCaseBox<TinyVecRepoBox<N>> {
    pub fn run_box(&self) -> Box<[u32]> {
        self.repo.fetch_box()
    }
}

// ---------- Bench helpers ----------
fn bench_owned_moves_for<const N: usize>(c: &mut Criterion) {
    let vec_uc = UseCase::new(VecRepo::<N>);
    let small_uc = UseCase::new(SmallVecRepo::<N>);
    let tiny_uc = UseCase::new(TinyVecRepo::<N>);

    let mut g = c.benchmark_group(format!("owned_move_N={}", N));
    g.bench_function("Vec (move)", |b| {
        b.iter(|| {
            let v = vec_uc.run_vec();
            black_box(v.last().copied())
        })
    });
    g.bench_function("SmallVec (move)", |b| {
        b.iter(|| {
            let v = small_uc.run_smallvec();
            black_box(v.last().copied())
        })
    });
    g.bench_function("TinyVec (move)", |b| {
        b.iter(|| {
            let v = tiny_uc.run_tinyvec();
            black_box(v.last().copied())
        })
    });
    g.finish();
}

fn bench_box_moves_for<const N: usize>(c: &mut Criterion) {
    let vec_uc = UseCaseBox::new(VecRepoBox::<N>);
    let small_uc = UseCaseBox::new(SmallVecRepoBox::<N>);
    let tiny_uc = UseCaseBox::new(TinyVecRepoBox::<N>);

    let mut g = c.benchmark_group(format!("box_move_N={}", N));
    g.bench_function("Vec -> Box<[u32]>", |b| {
        b.iter(|| {
            let bx = vec_uc.run_box();
            black_box(bx.len())
        })
    });
    g.bench_function("SmallVec -> Box<[u32]>", |b| {
        b.iter(|| {
            let bx = small_uc.run_box();
            black_box(bx.len())
        })
    });
    g.bench_function("TinyVec -> Box<[u32]>", |b| {
        b.iter(|| {
            let bx = tiny_uc.run_box();
            black_box(bx.len())
        })
    });
    g.finish();
}

// ---------- Criterion entry ----------
fn benches(c: &mut Criterion) {
    for &n in &[10, 100, 1_000, 10_000, 1_000_000] {
        match n {
            10 => {
                bench_owned_moves_for::<10>(c);
                bench_box_moves_for::<10>(c);
            }
            100 => {
                bench_owned_moves_for::<100>(c);
                bench_box_moves_for::<100>(c);
            }
            1_000 => {
                bench_owned_moves_for::<1_000>(c);
                bench_box_moves_for::<1_000>(c);
            }
            10_000 => {
                bench_owned_moves_for::<10_000>(c);
                bench_box_moves_for::<10_000>(c);
            }
            1_000_000 => {
                bench_owned_moves_for::<1_000_000>(c);
                bench_box_moves_for::<1_000_000>(c);
            }
            _ => unreachable!(),
        }
    }
}

criterion_group!(benches_group, benches);
criterion_main!(benches_group);
