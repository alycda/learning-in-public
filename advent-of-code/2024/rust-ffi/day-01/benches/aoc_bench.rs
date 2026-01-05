use day_01::{SAMPLE_INPUT, process_rust, process_c};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_process_rust(c: &mut Criterion) {
    c.bench_function("process rust", |b| {
        b.iter(|| process_rust(black_box(SAMPLE_INPUT)))
    });
}
fn benchmark_process_c(c: &mut Criterion) {
    c.bench_function("process c", |b| {
        b.iter(|| process_c(black_box(SAMPLE_INPUT)))
    });
}

criterion_group!(benches, benchmark_process_rust, benchmark_process_c);
criterion_main!(benches);