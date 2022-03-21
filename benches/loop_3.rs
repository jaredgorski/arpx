use arpx::Runtime;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::env::current_dir;

pub fn loop_3_benchmark(c: &mut Criterion) {
    let mut fixture_path = current_dir().unwrap();
    fixture_path.push("benches");
    fixture_path.push("fixtures");
    fixture_path.push("loop_3");
    fixture_path.push("loop_3");
    fixture_path.set_extension("yaml");

    c.bench_function("loop 3", |b| {
        b.iter(|| {
            Runtime::from_profile(
                black_box(&fixture_path.display().to_string()[..]),
                black_box(&["bench".to_string()]),
            )
            .unwrap()
            .run()
        })
    });
}

pub fn concurrent_loop_3_benchmark(c: &mut Criterion) {
    let mut fixture_path = current_dir().unwrap();
    fixture_path.push("benches");
    fixture_path.push("fixtures");
    fixture_path.push("loop_3");
    fixture_path.push("concurrent");
    fixture_path.set_extension("yaml");

    c.bench_function("concurrent loop 3", |b| {
        b.iter(|| {
            Runtime::from_profile(
                black_box(&fixture_path.display().to_string()[..]),
                black_box(&["bench".to_string()]),
            )
            .unwrap()
            .run()
        })
    });
}

pub fn loop_3_with_log_monitor_benchmark(c: &mut Criterion) {
    let mut fixture_path = current_dir().unwrap();
    fixture_path.push("benches");
    fixture_path.push("fixtures");
    fixture_path.push("loop_3");
    fixture_path.push("with_log_monitor");
    fixture_path.set_extension("yaml");

    c.bench_function("loop 3 with log monitor", |b| {
        b.iter(|| {
            Runtime::from_profile(
                black_box(&fixture_path.display().to_string()[..]),
                black_box(&["bench".to_string()]),
            )
            .unwrap()
            .run()
        })
    });
}

pub fn concurrent_loop_3_with_log_monitor_benchmark(c: &mut Criterion) {
    let mut fixture_path = current_dir().unwrap();
    fixture_path.push("benches");
    fixture_path.push("fixtures");
    fixture_path.push("loop_3");
    fixture_path.push("concurrent_with_log_monitor");
    fixture_path.set_extension("yaml");

    c.bench_function("concurrent loop 3 with log monitor", |b| {
        b.iter(|| {
            Runtime::from_profile(
                black_box(&fixture_path.display().to_string()[..]),
                black_box(&["bench".to_string()]),
            )
            .unwrap()
            .run()
        })
    });
}

criterion_group!(
    benches,
    loop_3_benchmark,
    concurrent_loop_3_benchmark,
    loop_3_with_log_monitor_benchmark,
    concurrent_loop_3_with_log_monitor_benchmark
);
criterion_main!(benches);
