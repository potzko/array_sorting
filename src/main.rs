#![allow(dead_code)]
#![allow(unused_imports)]

use rand::Rng;
use std::time::Instant;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

/*---------------------
    helper function
---------------------*/

fn generate_random_vec(n: usize) -> Vec<i64> {
    //returns a vector of length n, filled with random ints
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rng.gen());
    }
    vec
}

fn generate_random_vec_small(n: usize) -> Vec<i64> {
    //returns a vector of length n, filled with random ints
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rng.gen_range(0..1000));
    }
    vec
}

fn compare_vecs(a: &Vec<i64>, b: &Vec<i64>) -> bool{
    //compares the contents and order of two vecs
    if a.len() != b.len(){
        return false;
    }
    for i in 0..a.len(){
        if a[i] != b[i]{
            return false
        }
    }
    return true
}

fn std_sort_wraper(arr: &[i64]) -> Vec<i64>{
    let mut ret = arr.to_vec();
    ret.sort();
    //ret.sort_unstable();
    ret
}

/*---------------------
    merge sort
---------------------*/

fn combine_sorted_vecs(a: &[i64], b: &[i64]) -> Vec<i64>{
    //gets two sorted vecs, and returns one sorted vec, containing the values of both
    let mut new_arr = Vec::<i64>::with_capacity(a.len() + b.len());
    let (mut index_a, mut index_b) = (0,0);
    while index_a < a.len() && index_b < b.len(){
        if a[index_a] < b[index_b]{
            new_arr.push(a[index_a]);
            index_a += 1;
        }   else    {
            new_arr.push(b[index_b]);
            index_b += 1;
        }
    }
    match index_a{
        len if len == a.len() => new_arr.extend(&b[index_b..]),
        _ => new_arr.extend(&a[index_a..])
    };
    new_arr
}

fn merge_sort(arr: &[i64]) -> Vec<i64>{
    //gets a vec, returns a sorted vec, uses merge sort algorithem
    if arr.len() <= 2{
        let mut ret_arr = Vec::from(arr);
        ret_arr.sort();
        return ret_arr;
    }
    let (left,right) = arr.split_at(arr.len()/2);
    let sorted_arr = combine_sorted_vecs(
        &merge_sort(left),
        &merge_sort(right));
    sorted_arr
}


fn combine_sorted_vecs_inplace(a: &[i64], b: &[i64], tmp: &mut [i64]){
    let mut index_a = 0;
    let mut index_b = 0;
    let mut index_c = 0;
    while index_a < a.len() && index_b < b.len(){
        if a[index_a] < b[index_b]{
            tmp[index_c] = a[index_a];
            index_c += 1;
            index_a += 1;
        } else {
            tmp[index_c] = b[index_b];
            index_c += 1;
            index_b += 1;
        }
    }
    match a.len() - index_a{
        0 => tmp[index_c..].copy_from_slice(&b[index_b..]),
        _ => tmp[index_c..].copy_from_slice(&a[index_a..])
    };
}   

fn merge_sort_inplace(arr: &mut [i64]){
    let mut tmp: Vec<i64> = arr.to_vec();
    merge_sort_inplace_recursive(&mut arr[..], &mut tmp[..]);
}

fn merge_sort_inplace_recursive(arr: &mut [i64], tmp: &mut [i64]){
    if arr.len() < 2{
        return;
    }

    let (mut left, mut right) = arr.split_at_mut(arr.len()/2);
    let (mut left_tmp, mut right_tmp) = tmp.split_at_mut(tmp.len()/2);

    merge_sort_inplace_recursive(&mut left_tmp, &mut left);
    merge_sort_inplace_recursive(&mut right_tmp, &mut right);
    combine_sorted_vecs_inplace(left_tmp,right_tmp,arr);
}

fn merge_sort_inplace_wraper(arr: &[i64]) -> Vec<i64>{
    let mut ret: Vec<i64> = arr.to_vec();
    merge_sort_inplace(&mut ret);
    ret
}

/*---------------------
   radix sort base 16
---------------------*/

fn get_nth_hex_digit(num: &i64, digit: u32) -> usize{
    //gets the nth digit of the number in hexadecimal 
    ((num & (15 << digit*4)) >> digit*4).try_into().unwrap()
}

fn radix_sort(arr: &[i64]) -> Vec<i64>{
    //gets a vec and returns a sorted vec using the radix sorting algorithem 
    let mut negatives = Vec::<i64>::new();
    let mut positives = Vec::<i64>::new();
    for &i in arr{
        if i < 0{
            negatives.push(-i);
        }
        else{
            positives.push(i)
        }
    }
    negatives = radix_sort_recursive(&negatives, 15).into_iter().rev().map(|i| -i).collect();
    negatives.extend(radix_sort_recursive(&positives, 15));
    negatives
}

fn radix_sort_recursive(arr: &[i64], digit: u32) -> Vec<i64>{
    //gets a vec of numbers and a digit returns a sorted vec,
    //numbers must be positive, and sorted up to that digit
    if arr.len() == 0 || arr.len() == 1{
        return Vec::<i64>::from(arr);
    }
    let mut arrs: [Vec::<i64>; 16] = Default::default();
    for &i in arr{
        arrs[get_nth_hex_digit(&i,digit)].push(i);
    }
    if digit == 0{
        for i in 1..arrs.len(){
            arrs[0].extend(&arrs[i].clone());
        }
        return arrs[0].clone();
    }
    arrs[0] = radix_sort_recursive(&arrs[0],digit -1);
    for i in 1..arrs.len(){
        arrs[0].extend(radix_sort_recursive(&arrs[i], digit - 1));
    }
    
    arrs[0].clone()
}


/*---------------------
   radix sort base 256
---------------------*/
const EMPTY_VEC: Vec<i64> = Vec::new();
fn get_nth_base256_digit(num: &i64, digit: u32) -> usize{
    //gets the nth digit of the number in hexadecimal 
    ((num & (255 << digit*8)) >> digit*8).try_into().unwrap()
}

fn radix_sort_256(arr: &[i64]) -> Vec<i64>{
    //gets a vec and returns a sorted vec using the radix sorting algorithem 
    let mut negatives = Vec::<i64>::new();
    let mut positives = Vec::<i64>::new();
    for &i in arr{
        if i < 0{
            negatives.push(-i);
        }
        else{
            positives.push(i)
        }
    }
    negatives = radix_sort_256_recursive(&negatives, 7).into_iter().rev().map(|i| -i).collect();
    negatives.extend(radix_sort_256_recursive(&positives, 7));
    negatives
}

fn radix_sort_256_recursive(arr: &[i64], digit: u32) -> Vec<i64>{
    //gets a vec of numbers and a digit returns a sorted vec,
    //numbers must be positive, and sorted up to that digit
    if arr.len() == 0 || arr.len() == 1{
        return Vec::<i64>::from(arr);
    }
    let mut arrs: [Vec<i64>; 256] = [EMPTY_VEC; 256];
    for &i in arr{
        arrs[get_nth_base256_digit(&i,digit)].push(i);
    }
    if digit == 0{
        for i in 1..arrs.len(){
            arrs[0].extend(&arrs[i].clone());
        }
        return arrs[0].clone();
    }
    arrs[0] = radix_sort_256_recursive(&arrs[0],digit -1);
    for i in 1..arrs.len(){
        arrs[0].extend(radix_sort_256_recursive(&arrs[i], digit - 1));
    }
    
    arrs[0].clone()
}


/*---------------------
    quick sorts
---------------------*/

fn quick_sort(arr: &[i64]) -> Vec<i64>{
    if arr.len() <= 1{
        return  Vec::from(arr);
    }
    let mut lower = Vec::<i64>::with_capacity(arr.len()/3);
    let mut higher = Vec::<i64>::with_capacity(arr.len()/3);
    for &i in arr.iter().skip(1){
        if i <= arr[0]{
            lower.push(i);
        } else {
            higher.push(i);
        }
    } 
    lower = quick_sort(&lower);
    higher = quick_sort(&higher);
    lower.push(arr[0]);
    lower.extend(higher);
    lower
}


fn quick_sort_inplace(arr: &mut [i64]){
    if arr.len() < 2{
        return;
    }
    arr.swap(0, arr.len()/2);
    
    let mut pivot_index = 0;
    let mut known_sorted = arr.len() - 1;

    while pivot_index < known_sorted - 1{
        if arr[pivot_index + 1] <= arr[pivot_index]{
            arr.swap(pivot_index, pivot_index + 1);
            pivot_index += 1;
        }
        else {
            arr.swap(pivot_index + 1, known_sorted);
            known_sorted -= 1;
        }
    }
    if arr[known_sorted] < arr[pivot_index]{
        arr.swap(pivot_index, known_sorted);
        pivot_index += 1;
    }

    if arr.len() == 2{
        return;
    }

    quick_sort_inplace(&mut arr[0..pivot_index+1]);
    quick_sort_inplace(&mut arr[pivot_index+1..]);
}

fn quick_sort_inplace_wrapper(arr: &[i64]) -> Vec<i64>{
    let mut ret: Vec<i64> = arr.to_vec();
    quick_sort_inplace(&mut ret);
    ret
}

/*---------------------
    heap sorts
---------------------*/
fn heap_sort_inplace_wrapper(arr: &[i64]) -> Vec<i64>{
    let mut ret: Vec<i64> = arr.to_vec();
    heap_sort_inplace(&mut ret);
    ret
}

fn heap_sort_inplace(arr: &mut [i64]) {
    first_heapify(arr);
    for sorted_index in (1..arr.len()).rev() {
        arr.swap(sorted_index, 0);
        heapify(arr, 0, sorted_index - 1);
    }
}

fn first_heapify(arr: &mut [i64]){
    for start in (0..arr.len() / 2).rev() {
        heapify(arr, start, arr.len() - 1);
    }
}

fn heapify(arr: &mut [i64], start: usize, end: usize) {
    let mut root = start;
    let mut lower_child = root * 2 + 1; //left branch
    while !(lower_child > end) {
        if lower_child < end && arr[lower_child] < arr[lower_child + 1] {
            lower_child += 1;}          //swap to right branch
        if arr[root] < arr[lower_child] {
            arr.swap(root, lower_child);
            root = lower_child;
        } else {
            break;}
        lower_child = root * 2 + 1;
    }
}

/*---------------------
    shell sorts
---------------------*/ 

fn shell_sort(arr: &mut [i64]){
    let mut jump = arr.len()/2;
    while jump > 0{
        for index in (0..arr.len()).step_by(jump){
            let mut ind = index;
            let val = arr[index];
            while jump <= ind && arr[ind - jump] > val{
                arr.swap(ind, ind - jump);
                ind -= jump;
            }
            arr[ind] = val;
        }   
        jump /= 2;
    }
}

fn shell_sort_wrapper(arr: &[i64]) -> Vec<i64>{
    let mut tmp = arr.to_vec();
    shell_sort(&mut tmp);
    tmp
}


fn shell_sort_knuth(arr: &mut [i64]){
    let mut jump: usize = 1;
    let mut iter = 1;
    while jump < arr.len(){
        for index in (0..arr.len()).step_by(jump){
            let mut ind = index;
            let val = arr[index];
            while jump <= ind && arr[ind - jump] > val{
                arr[ind] = arr[ind - jump];
                ind -= jump;
            }
            arr[ind] = val;
        }   
        iter += 1;
        jump = (3_usize.pow(iter.try_into().unwrap()) - 1) / 2;
    }
}

fn shell_sort_knuth_wrapper(arr: &[i64]) -> Vec<i64>{
    let mut tmp = arr.to_vec();
    shell_sort_knuth(&mut tmp);
    tmp
}

/*---------------------
    bubbble sort
---------------------*/

fn bubble_sort(arr: &[i64]) -> Vec<i64>{
    let mut arr = Vec::from(arr);
    for _ in 0..arr.len(){
        for i in 1..arr.len(){
            if arr[i] < arr[i-1]{
                arr.swap(i,i-1)
            }
        }
    }
    arr
}

/*---------------------
    insertion sort
---------------------*/

fn insertion_sort(arr: &[i64]) -> Vec<i64>{
    let mut arr = Vec::from(arr);
    for index in 1..arr.len(){
        for i in (1..=index).rev(){
            if arr[i] < arr[i-1]{
                arr.swap(i,i-1)
            }
        }
    }
    arr
}

/*---------------------
    test functions
---------------------*/
fn check_sort<F>(sorter: F) -> bool
where F: Fn(&[i64]) -> Vec<i64>,
{
    let mut flag = true;
    let arr = generate_random_vec(1);
    let mut sorted_arr = arr.clone();
    sorted_arr.sort();
    flag = flag && compare_vecs(&sorted_arr, &sorter(&arr));

    for _ in 0..10{
        let arr = generate_random_vec(3);
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        flag = flag && compare_vecs(&sorted_arr, &sorter(&arr));
    }
    for _ in 0..10{
        let arr = generate_random_vec(10);
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        flag = flag && compare_vecs(&sorted_arr, &sorter(&arr));
    }
    for _ in 0..10{
        let arr = generate_random_vec(100);
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        flag = flag && compare_vecs(&sorted_arr, &sorter(&arr));
    }
    for _ in 0..3{
        let arr = generate_random_vec(1000);
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        flag = flag && compare_vecs(&sorted_arr, &sorter(&arr));
    }
    for _ in 0..1{
        let arr = generate_random_vec(10000);
        let mut sorted_arr = arr.clone();
        sorted_arr.sort();
        flag = flag && compare_vecs(&sorted_arr, &sorter(&arr));
    }
    flag
}

fn test_one_sort<F>(arr: &[i64], sorter: F) -> (std::time::Duration, Vec<i64>)
where F: Fn(&[i64]) -> Vec<i64>,
{
    let start = Instant::now();
    let a = sorter(arr);
    (start.elapsed(),a)
}

fn test_sorts(count: usize) {
    let do_slow_sorts = true;

    let odd_vec = generate_random_vec(count);
    // std sort
    let (time_std, _sorted_odd_vec) = test_one_sort(&odd_vec, std_sort_wraper);
    println!("std sort: {:?}",time_std);
    // N log(N) sorts
    let (time_merge, _sorted_vec_merge) = test_one_sort(&odd_vec, std_sort_wraper);
    println!("merge sort: {:?}",time_merge);
    let (time_merge_inplace, _sorted_vec_merge_inplace) = test_one_sort(&odd_vec, merge_sort_inplace_wraper);
    println!("merge sort inplace: {:?}",time_merge_inplace);
    let (time_quick, _sorted_vec_quick) = test_one_sort(&odd_vec, quick_sort);
    println!("quick sort: {:?}",time_quick);
    let (time_quick_inplace, _sorted_vec_quick_inplace) = test_one_sort(&odd_vec, quick_sort_inplace_wrapper);
    println!("quick sort inplace: {:?}",time_quick_inplace);
    let (time_radix, _sorted_vec_redix) = test_one_sort(&odd_vec, radix_sort);
    println!("redix sort: {:?}",time_radix);
    let (time_heap, _sorted_vec_heap) = test_one_sort(&odd_vec, heap_sort_inplace_wrapper);
    println!("heap sort: {:?}",time_heap);


    if do_slow_sorts{
        let (time_shell, _sorted_vec_shell) = test_one_sort(&odd_vec, shell_sort_wrapper);
        println!("shell sort: {:?}",time_shell);
        let (time_shell_knuth, _sorted_vec_shell_knuth) = test_one_sort(&odd_vec, shell_sort_knuth_wrapper);
        println!("shell sort knuth: {:?}",time_shell_knuth);
        let (time_radix_256, _sorted_vec_redix_256) = test_one_sort(&odd_vec, radix_sort);
        println!("redix base 256 sort: {:?}",time_radix_256);
        let (time_bubble, _sorted_vec_bubble) = test_one_sort(&odd_vec, bubble_sort);
        println!("bubble sort: {:?}",time_bubble);
        let (time_insertion, _sorted_vec_insert) = test_one_sort(&odd_vec, insertion_sort);
        println!("insertion sort: {:?}",time_insertion);
    }
}

fn test_sorts2(do_slow_sorts: bool)
{
    let mut lengths = vec![10, 100, 10_000, 50_000, 500_000, 5_000_000, 10_000_000]; // Change this as needed
    let mut sort_functions: Vec<fn(&[i64]) -> Vec<i64>> = vec![std_sort_wraper, merge_sort, merge_sort_inplace_wraper, quick_sort, quick_sort_inplace_wrapper, heap_sort_inplace_wrapper, radix_sort];
    let mut names = vec!["std sort", "merge sort", "merge sort inplace", "quick sort", "quick sort inplace", "heap sort inplace", "radix sort base 16"];

    if do_slow_sorts{
        sort_functions.extend([radix_sort_256, shell_sort_wrapper, shell_sort_knuth_wrapper, insertion_sort, bubble_sort]);
        names.extend(["radix sort base 256", "shell sort", "shell sort knuth", "insertion sort", "bubble sort"]);
        lengths = lengths[0..4].to_vec();
    }

    println!("starting sort checks");
    let elem_count = lengths[lengths.len()-1];
    for i in 0..sort_functions.len(){
        println!("checking {}", names[i]);
        let algorithem = sort_functions[i];
        let mut total_over_algorithem = Duration::new(0,0);
        for i in &lengths{
            let mut total_over_section = Duration::new(0,0);
            for _ in 0..elem_count/i{
                let tmp = generate_random_vec(*i);
                let start = Instant::now();
                algorithem(&tmp);
                let start = start.elapsed();
                total_over_section += start;
            }
            total_over_algorithem += total_over_section;
            println!("took {: <13?} seconds to sort {: <13} arrays with length {: <13}, avraging {: <13?} per array", total_over_section, elem_count/i, i, total_over_section/((elem_count/i)).try_into().unwrap());
        }
        println!{"took in total: {:?}", total_over_algorithem};
    }
}

fn main(){
    
}