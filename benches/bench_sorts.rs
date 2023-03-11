use array_sorting::{
    bubble_sort, generate_random_vec, heap_sort_inplace_wrapper, insertion_sort, merge_sort,
    merge_sort_bottom_up_inplace_wrapper, merge_sort_inplace_wraper, quick_sort,
    quick_sort_inplace_wrapper, radix_sort, radix_sort_256, shell_sort_knuth_wrapper,
    shell_sort_wrapper, std_sort_wraper,
};
use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BatchSize, BenchmarkGroup, BenchmarkId,
    Criterion,
};
use std::time::Duration;

pub fn bench_one<F>(size: usize, sorter: F, name: String, group: &mut BenchmarkGroup<WallTime>)
where
    F: Fn(&[i64]) -> Vec<i64>,
{
    group.bench_with_input(BenchmarkId::new(name, size), &size, |b, &size| {
        b.iter_batched_ref(
            || generate_random_vec(size),
            |data| sorter(data),
            BatchSize::SmallInput,
        )
    });
}

fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sorts");
    group.warm_up_time(Duration::from_secs_f32(0.2));
    group.sample_size(10000);

    let sorts_fast: Vec<fn(&[i64]) -> Vec<i64>> = vec![
        heap_sort_inplace_wrapper,
        merge_sort,
        merge_sort_inplace_wraper,
        quick_sort,
        quick_sort_inplace_wrapper,
        radix_sort,
        radix_sort_256,
        std_sort_wraper,
        merge_sort_bottom_up_inplace_wrapper,
    ];
    let slow_sorts: Vec<fn(&[i64]) -> Vec<i64>> = vec![
        insertion_sort,
        bubble_sort,
        shell_sort_knuth_wrapper,
        shell_sort_wrapper,
    ];

    let names_fast: Vec<String> = [
        "heap_sort_inplace_wrapper",
        "merge_sort",
        "merge_sort_inplace_wraper",
        "quick_sort",
        "quick_sort_inplace_wrapper",
        "radix_sort",
        "radix_sort_256",
        "std_sort_wraper",
        "merge_sort_bottom_up_inplace_wrapper",
    ]
    .map(String::from)
    .to_vec();
    let names_slow: Vec<String> = [
        "insertion_sort",
        "bubble_sort",
        "shell_sort_knuth_wrapper",
        "shell_sort_wrapper",
    ]
    .map(String::from)
    .to_vec();

    let start: usize = 44;
    let end: usize = 44;
    let resolution: usize = 1;
    let array_sizes = (start..=end).step_by(resolution);

    let mut sort_list = sorts_fast.clone();
    let mut names = names_fast.clone();
    if end <= 50_000 {
        sort_list.extend(slow_sorts.iter());
        names.extend(names_slow);
    }
    for i in 0..50{
        bench_one(i, shell_sort_knuth_wrapper, "Knuth".to_string(), &mut group);
    }
    /*
    for size in array_sizes {
        print!("a");
        for (sorter, name) in sort_list.iter().zip(names.iter()) {
            bench_one(size, sorter, name.to_string(), &mut group);
        }
    }*/
    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
