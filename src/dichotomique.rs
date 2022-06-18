pub fn dichotomique<T>(arr: &[T], searched_value: T) -> Option<usize>
where
    T: PartialOrd + PartialEq,
{
    if arr.is_empty() {
        return None;
    }

    let mut begin: usize = 0;
    let mut end: usize = arr.len() - 1;
    let mut middle: usize;

    while begin <= end {
        middle = (begin + end) / 2;

        if arr[middle] == searched_value {
            return Some(middle);
        } else if arr[middle] < searched_value {
            begin = middle + 1;
        } else {
            end = middle - 1;
        }
    }

    None
}

#[test]
fn should_return_none_when_empty_array() {
    let index = dichotomique(&[], 30);
    assert_eq!(None, index);
}
#[test]
fn should_return_none_when_value_not_found() {
    let index = dichotomique(&[1, 2, 3, 4, 5], 30);
    assert_eq!(None, index);
}
#[test]
fn should_return_index_of_found_value_which_is_less_than_the_middle_value_of_array() {
    let index = dichotomique(&[1, 2, 3, 4, 5], 2);
    assert_eq!(Some(1), index);
}
#[test]
fn should_return_index_of_found_value_which_is_greater_than_the_middle_value_of_array() {
    let index = dichotomique(&[1, 2, 3, 4, 5], 4);
    assert_eq!(Some(3), index);
}
#[test]
fn should_return_index_of_the_first_value_when_found() {
    let index = dichotomique(&[1, 2, 3, 4, 5], 1);
    assert_eq!(Some(0), index);
}
#[test]
fn should_return_index_of_the_last_value_when_found() {
    let index = dichotomique(&[1, 2, 3, 4, 5], 5);
    assert_eq!(Some(4), index);
}
