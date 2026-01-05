use aoc_ffi_day01::{SAMPLE_INPUT, process_part1, process_part2, process_part1_c};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_process_part1_rust(c: &mut Criterion) {
    c.bench_function("process rust part 1", |b| {
        b.iter(|| process_part1(black_box(SAMPLE_INPUT)))
    });
}

fn benchmark_process_part1_c(c: &mut Criterion) {
    c.bench_function("process c part 1", |b| {
        b.iter(|| process_part1_c(black_box(SAMPLE_INPUT)))
    });
}

fn benchmark_process_part2_rust(c: &mut Criterion) {
    c.bench_function("process rust part 2", |b| {
        b.iter(|| process_part2(black_box(SAMPLE_INPUT)))
    });
}

criterion_group!(benches, benchmark_process_part1_rust, benchmark_process_part1_c, benchmark_process_part2_rust);
criterion_main!(benches);