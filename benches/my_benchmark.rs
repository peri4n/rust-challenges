use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use leetcode::leetcode::solutions::p0001_two_sum::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn random_vector(length: usize) -> Vec<i32> {
    let mut rng = StdRng::seed_from_u64(1000);
    let mut res = Vec::with_capacity(length);

    for _ in 0..length {
        res.push(rng.gen_range(0..1000));
    }

    res
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let random_vec = random_vector(100000);

    c.bench_with_input(BenchmarkId::new("smart", 1000), &random_vec, |b, v| {
        b.iter(|| two_sum1(v, black_box(49)))
    });
    c.bench_with_input(BenchmarkId::new("stupid", 1000), &random_vec, |b, v| {
        b.iter(|| two_sum3(v, black_box(49)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
