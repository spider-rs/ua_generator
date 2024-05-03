use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ua_generator::ua::spoof_ua;

/// bench spoof_ua randomizer
pub fn bench_speed(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate/personal");
    let sample_count = 1000;
    let sample_title = format!("generate {} samples", sample_count);

    group.sample_size(sample_count);
    group.bench_function(format!("basic: {}", sample_title), |b| {
        b.iter(|| black_box(spoof_ua()))
    });
    group.finish();
}

criterion_group!(benches, bench_speed);
criterion_main!(benches);
