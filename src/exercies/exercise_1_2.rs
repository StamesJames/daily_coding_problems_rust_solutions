fn locate_smallest_window_to_be_sort<T>(arr:&[T])-> (usize, usize)
where
    T: PartialOrd + Clone
{
    let mut result = (0,0);
    if arr.len() < 2 {return result;}
    let mut current = &arr[0];
    for i in 1..arr.len(){
        if *current > arr[i] {
            result.1 = i;
        } else {
            current = &arr[i]
        }
    }
    current = &arr[arr.len()-1];
    for i in (0..arr.len()-1).rev(){
        if *current < arr[i]{
            result.0 = i;
        }else {
            current = &arr[i];
        }
    }
    result
}

#[test]
fn test_3_7_5_6_9(){
    assert_eq!((1,3), locate_smallest_window_to_be_sort(&[3,7,5,6,9]))
}

#[test]
fn test_1_2_3_100_4_5_6_100_7_8_9(){
    assert_eq!((3,10), locate_smallest_window_to_be_sort(&[1,2,3,100,4,5,6,100,7,8,9]))
}

#[test]
fn test_9_8_7_6_5_4_3_2_1(){
    assert_eq!((0,8), locate_smallest_window_to_be_sort(&[9,8,7,6,5,4,3,2,1]));
}

fn test_100_1_2_3_4_5_0(){
    assert_eq!((0, 6), locate_smallest_window_to_be_sort(&[100,1,2,3,4,5,0]));
}



