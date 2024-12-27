use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::parking_lot::benches,
    benchmarks::std::benches
}
