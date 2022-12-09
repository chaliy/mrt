use criterion::{black_box, criterion_group, criterion_main, Criterion};

use mrt::change_detection::read_hashes;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("retest", |b| b.iter(|| {
        read_hashes("./examples/basic-sample");
        black_box(1);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);