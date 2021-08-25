use core::panic;
use std::ops::{Add};
use std::cmp::max;

fn calculate_maximum_subarray_sum_with_zero<T>(arr:&[T], zero: &T) -> T
where
    T: Ord + Add<Output = T> + Clone
{
    if arr.len() == 0 {return zero.clone();};
    let mut results: Vec<(T,T)> = Vec::with_capacity(arr.len());
    results.push((arr[0].clone(), arr[0].clone()));

    for (i,elem) in arr.iter().enumerate().skip(1){
        let (last_total, last_till) = &results[i-1];
        let current_till = max(last_till.clone() + elem.clone(), elem.clone());
        let current_total = max(last_total, max(zero, &current_till));
        results.push((current_total.clone(), current_till));
    }

    results.iter().fold(zero.clone(), |cur, (x,_)| if cur < *x {x.clone()} else {cur})
}

#[test]
fn test_34_n50_42_14_n5_86(){
    assert_eq!(137, calculate_maximum_subarray_sum_with_zero(&[35, -50, 42, 14, -5, 86], &0));
}

fn test_n5_n1_n8_n9(){
    assert_eq!(0, calculate_maximum_subarray_sum_with_zero(&[-5,-1,-8,-9], &0));
}