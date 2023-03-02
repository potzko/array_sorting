use branch_tests::{merge_sort, std_sort_wraper, generate_random_vec};
use criterion::Criterion{};

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorts");

    let sizes = [10, 100, 10_000, 50_000, 500_000, 5_000_000, 10_000_000];

    for size in sizes.iter() {
        let data = generate_random_vec(*size);
        group.bench_function(format!("Std sort {}", size), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |data| std_sort_wraper(data),
                BatchSize::SmallInput,
            )
        });

        group.bench_function(format!("Merge sort {}", size), |b| {
            b.iter_batched_ref(
                || data.clone(),
                |data| merge_sort(data),
                BatchSize::SmallInput,
            )
        });

        // Add more sort functions here

    }
    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
