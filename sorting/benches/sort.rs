use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorts");
    group.bench_function("Bubble Sort", |b| {
        b.iter(|| quick_sort::bubble_sort::bubble_sort(black_box(&mut vec![5, 4, 3, 2, 1])))
    });
    group.bench_function("Quick Sort", |b| {
        b.iter(|| quick_sort::quick_sort::quick_sort(black_box(&mut vec![5, 4, 3, 2, 1])))
    });
    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
