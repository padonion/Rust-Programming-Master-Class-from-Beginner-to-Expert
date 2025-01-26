use rs_benchmark::{sort_algo_1, sort_algo_2};
use criterion::{criterion_group, criterion_main, Criterion};

fn sort_benchmark(c: &mut Criterion) {
    let mut numbers = vec![
        1,2,3,6,5,4,8,5,6,4,98,6,2,1,45,6,2,5,5,4,1,6,5,0,5,8,40
    ];

    c.bench_function("Sorting algorithm", |b| {
        b.iter(|| sort_algo_2(&mut numbers))
    });
}

criterion_group!(benches, sort_benchmark);
criterion_main!(benches);