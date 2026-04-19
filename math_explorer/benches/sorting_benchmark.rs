use criterion::{Criterion, black_box, criterion_group, criterion_main};
use math_explorer::applied::algorithms::sorting::divide_conquer::{merge_sort, quick_sort};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorting");

    // Generate a random array of 1000 integers
    let mut data = Vec::with_capacity(1000);
    for i in 0..1000 {
        data.push(1000 - i); // worst case or randomish
    }

    group.bench_function("merge_sort", |b| b.iter(|| merge_sort(black_box(&data))));
    group.bench_function("quick_sort", |b| b.iter(|| quick_sort(black_box(&data))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
