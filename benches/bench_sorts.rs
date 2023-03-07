use array_sorting::{generate_random_vec, merge_sort, std_sort_wraper, merge_sort_inplace_wraper, quick_sort_inplace_wrapper, quick_sort, heap_sort_inplace_wrapper, radix_sort, radix_sort_256};
use criterion::BatchSize;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorts");

    let sizes: Vec<usize> = (0..=10_000_000).step_by(500_000).collect();

    for size in sizes.iter() {
        group.bench_with_input(BenchmarkId::new("Std sort", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| std_sort_wraper(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Merge sort", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| merge_sort(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Merge sort inplace", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| merge_sort_inplace_wraper(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Quick sort", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| quick_sort(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Quick sort inplace", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| quick_sort_inplace_wrapper(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Heap sort inplace", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| heap_sort_inplace_wrapper(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Radix sort base 64", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| radix_sort(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_with_input(BenchmarkId::new("Radix sort base 256", size), size, |b, &size| {
            b.iter_batched_ref(
                || generate_random_vec(size),
                |data| radix_sort_256(data),
                BatchSize::SmallInput,
            )
        });

        // Add more sort functions here
    }
    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
