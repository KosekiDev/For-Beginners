pub fn fizzbuzz(number: i8, funcs: Vec<fn(i8) -> Option<String>>) -> String {
    for f in funcs {
        if let Some(val) = f(number) {
            return val;
        }
    }

    if number % 15 == 0 {
        "fizzbuzz".to_owned()
    } else if number % 5 == 0 {
        "buzz".to_owned()
    } else if number % 3 == 0 {
        "fizz".to_owned()
    } else {
        number.to_string()
    }
}

#[test]
fn should_return_fizzbuzz_when_number_is_a_multiple_of_fifteen() {
    assert_eq!("fizzbuzz", fizzbuzz(30, vec![]));
}
#[test]
fn should_return_fiz_when_number_is_a_multiple_of_three() {
    assert_eq!("fizz", fizzbuzz(6, vec![]));
}
#[test]
fn should_return_buzz_when_number_is_a_multiple_of_five() {
    assert_eq!("buzz", fizzbuzz(10, vec![]));
}
#[test]
fn should_return_the_number_when_number_is_not_a_multiple_of_three_five_or_fifteen() {
    assert_eq!("2", fizzbuzz(2, vec![]));
}
#[test]
fn should_return_the_fuzz_when_number_is_a_multiple_of_three() {
    assert_eq!(
        "fuzz",
        fizzbuzz(
            6,
            vec![|number: i8| -> Option<String> {
                match number % 3 {
                    0 => Some("fuzz".to_string()),
                    _ => None,
                }
            }]
        )
    );
}
#[test]
fn should_return_the_bazz_when_number_is_a_multiple_of_five() {
    assert_eq!(
        "bazz",
        fizzbuzz(
            10,
            vec![|number: i8| -> Option<String> {
                match number % 5 {
                    0 => Some("bazz".to_string()),
                    _ => None,
                }
            }]
        )
    );
}
#[test]
fn should_return_the_fazzbazz_when_number_is_a_multiple_of_fifteen_with_multiple_functions_sets() {
    assert_eq!(
        "fazzbazz",
        fizzbuzz(
            30,
            vec![
                |number: i8| -> Option<String> {
                    match number % 15 {
                        0 => Some("fazzbazz".to_string()),
                        _ => None,
                    }
                },
                |number: i8| -> Option<String> {
                    match number % 5 {
                        0 => Some("fazz".to_string()),
                        _ => None,
                    }
                },
                |number: i8| -> Option<String> {
                    match number % 3 {
                        0 => Some("bazz".to_string()),
                        _ => None,
                    }
                }
            ]
        )
    );
}
