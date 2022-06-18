// Gives the n number of the fibonacci's suite

pub fn fibonacci_memoization(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut left_number: usize = 0;
    let mut right_number: usize = 1;
    let mut next: usize = 0;

    for _ in 2..=n {
        next = left_number + right_number;
        left_number = right_number;
        right_number = next;
    }

    next
}

#[test]
fn should_return_0_for_the_first_number() {
    assert_eq!(0, fibonacci_memoization(0));
}
#[test]
fn should_return_1_for_the_second_number() {
    assert_eq!(1, fibonacci_memoization(1));
}

#[test]
fn should_return_1_for_the_third_number() {
    assert_eq!(1, fibonacci_memoization(2));
}

#[test]
fn should_return_2_for_the_fourth_number() {
    assert_eq!(2, fibonacci_memoization(3));
}

#[test]
fn should_return_34_for_the_tenth_number() {
    assert_eq!(34, fibonacci_memoization(9));
}
