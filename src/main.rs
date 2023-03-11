#![allow(dead_code)]
#![allow(unused_imports)]
use array_sorting::{
    bubble_sort, check_sort, generate_random_vec_small, merge_sort_inplace_wraper, std_sort_wraper,
    test_one_sort,shell_sort_knuth_32_items, merge_sort_inplace_early_stopping_wraper, generate_random_vec, merge_sort_bottom_up_with_early_stopping_inplace , merge_sort_bottom_up_inplace,
    dist_sort, quick_sort_inplace_early_stop_wrapper
};


fn merge_sort_bottom_up_with_early_stopping_inplace_wrapper(arr: &[i64]) -> Vec<i64> {
    let mut ret = arr.to_vec();
    merge_sort_bottom_up_inplace(&mut ret);
    ret
}

fn merge_sort_bottom_up_inplace_wrapper(arr: &[i64]) -> Vec<i64> {
    let mut ret = arr.to_vec();
    merge_sort_bottom_up_inplace(&mut ret);
    ret
}

fn std_unstable_wraper(arr: &[i64]) -> Vec<i64> {
    let mut ret = arr.to_vec();
    ret.sort_unstable();
    ret
}

fn dist_sort_wrapper(arr: &[i64]) -> Vec<i64> {
    let mut ret = arr.to_vec();
    dist_sort(&mut ret);
    ret
}

fn main() {
    println!("{:?}", test_one_sort(&generate_random_vec(5_000_000), quick_sort_inplace_early_stop_wrapper).0);
    println!("{:?}", test_one_sort(&generate_random_vec(50_000), dist_sort_wrapper).0);
    println!("{:?}", test_one_sort(&generate_random_vec_small(5_000_000), dist_sort_wrapper).0);
    println!("{:?}", test_one_sort(&generate_random_vec_small(5_000_000), merge_sort_inplace_early_stopping_wraper).0);
    println!("{:?}", test_one_sort(&generate_random_vec_small(5_000_000), std_sort_wraper).0);
}