pub fn bubble_sort<T>(arr: &mut [T])
where
    T: PartialOrd,
{
    if arr.len() < 2 {
        return;
    }

    let mut last_index = arr.len() - 2;
    let mut current_index: usize = 0;

    loop {
        while current_index <= last_index {
            if arr[current_index] > arr[current_index + 1] {
                arr.swap(current_index, current_index + 1);
            }

            current_index += 1;
        }

        current_index = 0;
        last_index = match last_index.checked_sub(1) {
            Some(val) => val,
            None => break,
        };
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
