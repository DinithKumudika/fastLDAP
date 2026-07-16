use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_dummy(c: &mut Criterion) {
    c.bench_function("dummy", |b| b.iter(|| {}));
}

criterion_group!(benches, benchmark_dummy);
criterion_main!(benches);
