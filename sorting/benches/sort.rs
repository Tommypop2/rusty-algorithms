use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
fn generate_random(length: u32, max: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<u32> = Vec::with_capacity(length as usize);
    for _ in 0..length {
        let integer = rng.gen_range(0..(max + 1));
        vec.push(integer)
    }
    vec
}
fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorts");
    let random_vector = generate_random(10000, 550);
    group.bench_function("Bubble Sort", |b| {
        b.iter(|| quick_sort::bubble_sort::bubble_sort(black_box(&mut random_vector.clone())))
    });
    group.bench_function("Quick Sort", |b| {
        b.iter(|| quick_sort::quick_sort::quick_sort(black_box(&mut random_vector.clone())))
    });
    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
