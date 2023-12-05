use criterion::{criterion_group, criterion_main, Criterion};
use day_04::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new_main", |b| b.iter(new_main));
    c.bench_function("old_main", |b| b.iter(old_main));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
