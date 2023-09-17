pub fn bubble_sort<T>(arr: &mut [T])
where
    T: PartialOrd + Copy,
{
    for i in 0..arr.len() {
        for j in 0..(arr.len() - i - 1) {
            if arr[j] > arr[j + 1] {
                let temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp
            }
        }
    }
}

#[test]
fn should_return_because_arrays_minimum_length_is_two() {
    let array = [4];
    let mut sorted_array = array.clone();
    bubble_sort(&mut sorted_array);

    assert_eq!(array, sorted_array);
}
#[test]
fn should_filter_correctly_in_ascending_order() {
    let mut array = [4, 29, 1, -4, 3, 7, 41, -1, 9, -51];
    bubble_sort(&mut array);

    assert_eq!([-51, -4, -1, 1, 3, 4, 7, 9, 29, 41], array);
}
