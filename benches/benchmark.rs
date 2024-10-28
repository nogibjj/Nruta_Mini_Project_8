// benches/benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_multiply_benchmark::multiply_values;

fn benchmark_multiply(c: &mut Criterion) {
    let values = vec![10.0; 1_000_000]; // Large array to see performance impact
    c.bench_function("multiply_values", |b| {
        b.iter(|| multiply_values(black_box(&values), black_box(2.0)))
    });
}

criterion_group!(benches, benchmark_multiply);
criterion_main!(benches);
