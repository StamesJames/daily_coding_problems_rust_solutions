// Of a given array of numbers calculate a array where at index i stands the product of all numbers in the array exept the one at index i

use std::ops::{Div, Mul};

fn prod_array_with_div<T, const N:usize>(arr: &[T;N]) -> [T;N]
where
    T: Mul<Output = T> + Div<Output = T> + Clone
{
    if arr.len() == 0 { return arr.clone(); }
    let prod = arr.iter().skip(1).fold(arr[0].clone(), |p, x| p * x.clone());
    arr.clone().map(|x| prod.clone() / x)
}

fn prod_array_without_div<T, const N:usize>(arr: &[T;N]) -> [T;N]
where
    T: Mul<Output = T> + Clone
{   
    if arr.len() == 0 {return arr.clone();}
    if arr.len() == 1 {
        panic!("The array must be at least 2 elements long, otherwise the 1 Element for a Generic multiplacitive Type T is not defined.")
    }
    let mut left_to_right = arr.clone();
    let mut right_to_left = arr.clone();
    for i in 1..arr.len() {
        left_to_right[i] = left_to_right[i].clone() * left_to_right[i-1].clone();
        right_to_left[arr.len()-1 - i] = right_to_left[arr.len()-1 - i].clone() * right_to_left[arr.len() -i].clone();
    };

    let mut result = arr.clone();
    result[0] = right_to_left[1].clone();
    result[arr.len()-1] = left_to_right[arr.len()-2].clone();
    for i in 1..result.len() -1 {
        result[i] = left_to_right[i-1].clone() * right_to_left[i+1].clone();
    }
    result
}

#[test]
fn test_1_2_3_4_5_with_div(){
    assert_eq!([120, 60, 40, 30, 24], prod_array_with_div(&[1,2,3,4,5]));
}


#[test]
fn test_3_2_1_with_div(){
    assert_eq!([2,3,6], prod_array_with_div(&[3,2,1]));
}


#[test]
fn test_1_2_3_4_5_without_div(){
    assert_eq!([120, 60, 40, 30, 24], prod_array_without_div(&[1,2,3,4,5]));
}


#[test]
fn test_3_2_1_without_div(){
    assert_eq!([2,3,6], prod_array_without_div(&[3,2,1]));
}


#[test]
#[should_panic]
fn test_one_element_without_div(){
    prod_array_without_div(&[1]);
}

#[test]
fn test_one_element_with_div(){
    assert_eq!([1], prod_array_with_div(&[1]));
}

#[test]
fn test_empty_with_div(){
    assert_eq!([] as [i8;0], prod_array_with_div::<i8,0>(&[]));
}

#[test]
fn test_empty_without_div(){
    assert_eq!([] as [i8;0], prod_array_without_div::<i8,0>(&[]));
}