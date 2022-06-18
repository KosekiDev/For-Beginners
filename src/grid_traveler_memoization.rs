// Gives the number of the different ways to go from the top left corner to the bottom right corner
// in an x by y grid using memoization technique for optimize the algorithm

use std::collections::HashMap;

pub fn grid_traveler_memoization(x: usize, y: usize, memo: &mut HashMap<String, usize>) -> usize {
    let key = format!("{} {}", x, y);

    if memo.contains_key(&key) {
        return memo[&key];
    }
    if x == 0 || y == 0 {
        return 0;
    }
    if x == 1 || y == 1 {
        return 1;
    }

    let new_x = grid_traveler_memoization(x - 1, y, memo);
    let new_y = grid_traveler_memoization(x, y - 1, memo);

    memo.insert(key.clone(), new_x + new_y);
    memo[&key]
}

#[test]
fn should_return_1() {
    assert_eq!(1, grid_traveler_memoization(1, 1, &mut HashMap::new()));
}
#[test]
fn should_return_3() {
    assert_eq!(3, grid_traveler_memoization(2, 3, &mut HashMap::new()));
}
#[test]
fn should_return_6() {
    assert_eq!(6, grid_traveler_memoization(3, 3, &mut HashMap::new()));
}
