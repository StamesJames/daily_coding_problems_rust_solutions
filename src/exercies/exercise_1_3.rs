use std::cmp::max;
use std::ops::Add;

fn calculate_maximum_subarray_sum_with_zero_with_wraparound<T>(arr: &[T], zero: &T) -> T
where
    T: Ord + Add<Output = T> + Clone,
{
    if arr.len() == 0 {
        return zero.clone();
    };
    let mut last_till = (0, arr[0].clone());
    let mut current_max = (
        if arr[0] > *zero { 0 } else { arr.len() },
        max(zero, &arr[0]).clone(),
    );

    for (i, elem) in arr.iter().enumerate().skip(1) {
        let last_plus_elem = last_till.1.clone() + elem.clone();
        let current_till = (
            if last_plus_elem > *elem { last_till.0 } else { i },
            max(last_plus_elem, elem.clone()),
        );
        current_max = (
            if current_till.1 > current_max.1 { current_till.0 } else { current_max.0 },
            max(&current_till.1, &current_max.1).clone(),
        );
        last_till = current_till;
    }

    for (i, elem) in arr.iter().enumerate(){
        if i >= current_max.0 || i == arr.len() || i >= last_till.0 {break;};
        let last_plus_elem = last_till.1.clone() + elem.clone();
        let current_till = (
            if last_plus_elem > *elem { last_till.0 } else { i },
            max(last_plus_elem, elem.clone()),
        );
        current_max = (
            if current_till.1 > current_max.1 { current_till.0 } else { current_max.0 },
            max(&current_till.1, &current_max.1).clone(),
        );
        last_till = current_till;
    }
    current_max.1.clone()
}

fn calculate_maximum_subarray_sum_with_zero<T>(arr: &[T], zero: &T) -> T
where
    T: Ord + Add<Output = T> + Clone,
{
    if arr.len() == 0 {
        return zero.clone();
    };
    let mut last_till = arr[0].clone();
    let mut current_max = max(zero, &arr[0]).clone();
    for elem in arr.iter().skip(1) {
        let current_till = max(last_till.clone() + elem.clone(), elem.clone());
        current_max = max(&current_till, &current_max).clone();
        last_till = current_till;
    }

    current_max.clone()
}

#[test]
fn test_35_n50_42_14_n5_86() {
    assert_eq!(
        137,
        calculate_maximum_subarray_sum_with_zero(&[35, -50, 42, 14, -5, 86], &0)
    );
}


#[test]
fn test_35_50_42_14_5_86() {
    assert_eq!(
        35+50+42+14+5+86,
        calculate_maximum_subarray_sum_with_zero(&[35, 50, 42, 14, 5, 86], &0)
    );
}

#[test]
fn test_n5_n1_n8_n9() {
    assert_eq!(
        0,
        calculate_maximum_subarray_sum_with_zero(&[-5, -1, -8, -9], &0)
    );
}

#[test]
fn test_epmty() {
    assert_eq!(0, calculate_maximum_subarray_sum_with_zero(&[], &0));
}

#[test]
fn test_one_element() {
    assert_eq!(10, calculate_maximum_subarray_sum_with_zero(&[10], &0));
}

#[test]
fn test_one_element_negative() {
    assert_eq!(0, calculate_maximum_subarray_sum_with_zero(&[-10], &0));
}


#[test]
fn test_35_n50_42_14_n5_86_with_wrap() {
    assert_eq!(
        137 + 35,
        calculate_maximum_subarray_sum_with_zero_with_wraparound(&[35, -50, 42, 14, -5, 86], &0)
    );
}

#[test]
fn test_35_50_42_14_5_86_with_wrap() {
    assert_eq!(
        35+50+42+14+5+86,
        calculate_maximum_subarray_sum_with_zero_with_wraparound(&[35, 50, 42, 14, 5, 86], &0)
    );
}

#[test]
fn test_n5_n1_n8_n9_with_wrap() {
    assert_eq!(
        0,
        calculate_maximum_subarray_sum_with_zero_with_wraparound(&[-5, -1, -8, -9], &0)
    );
}

#[test]
fn test_epmty_with_wrap() {
    assert_eq!(0, calculate_maximum_subarray_sum_with_zero_with_wraparound(&[], &0));
}

#[test]
fn test_one_element_with_wrap() {
    assert_eq!(10, calculate_maximum_subarray_sum_with_zero_with_wraparound(&[10], &0));
}

#[test]
fn test_one_element_negative_with_wrap() {
    assert_eq!(0, calculate_maximum_subarray_sum_with_zero_with_wraparound(&[-10], &0));
}