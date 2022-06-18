// Calculates the greatest common denominator between two numbers

pub fn greatest_common_denominator(a: isize, b: isize) -> isize {
    if a == 0 || b == 0 {
        return 0;
    }

    let (higher, lower) = match a < b {
        true => (b, a),
        false => (a, b),
    };

    match higher % lower {
        0 => lower,
        rest => greatest_common_denominator(lower, rest),
    }
}

#[test]
fn shoud_return_2() {
    assert_eq!(2, greatest_common_denominator(2, 4));
}
#[test]
fn shoud_return_12() {
    assert_eq!(12, greatest_common_denominator(48, 60));
}
#[test]
fn shoud_return_none() {
    assert_eq!(0, greatest_common_denominator(0, 60));
}
