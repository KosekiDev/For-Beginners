// Return true if we can calculate the number with a sum of numbers contained in an array

use std::collections::HashMap;

pub fn can_sum_memoization(
    number: usize,
    array: &Vec<usize>,
    memo: &mut HashMap<usize, bool>,
) -> bool {
    if memo.contains_key(&number) {
        return memo[&number];
    }

    for i in 0..array.len() {
        let reminder = match number.checked_sub(array[i]) {
            Some(0) => return true,
            Some(result) => result,
            None => continue,
        };

        if can_sum_memoization(reminder, array, memo) {
            memo.insert(number, true);
            return true;
        }
    }

    memo.insert(number, false);
    false
}

#[test]
fn should_return_true_when_value_found_in_array() {
    assert_eq!(true, can_sum_memoization(7, &vec![7], &mut HashMap::new()));
}
#[test]
fn should_return_true_when_can_sum_memoization_with_two_numbers() {
    assert_eq!(
        true,
        can_sum_memoization(7, &vec![3, 4], &mut HashMap::new())
    );
}
#[test]
fn should_return_true_when_can_sum_memoization_with_three_numbers() {
    assert_eq!(
        true,
        can_sum_memoization(7, &vec![2, 1, 4], &mut HashMap::new())
    );
}
#[test]
fn should_return_true_with_big_number() {
    assert_eq!(
        true,
        can_sum_memoization(
            300,
            &vec![10, 3, 25, 88, 80, 1, 4, 200, 110, 80, 10],
            &mut HashMap::new()
        )
    );
}
#[test]
fn should_return_false() {
    assert_eq!(
        true,
        can_sum_memoization(47, &vec![10, 3, 25], &mut HashMap::new())
    );
}
