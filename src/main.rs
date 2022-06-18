mod bubble_sort;
mod can_sum_memoization;
mod dichotomique;
mod fibonacci;
mod fizzbuzz;
mod grid_traveler_memoization;

use std::collections::HashMap;

use bubble_sort::*;
use can_sum_memoization::*;
use dichotomique::*;
use fibonacci::*;
use fizzbuzz::*;
use grid_traveler_memoization::*;

fn main() {
    let mut arr = [1, 6, 4, 9, -5];
    bubble_sort(&mut arr);

    println!("Bubble sort   > {:?}", arr);
    println!("Dichotomique  > {:?}", dichotomique(&arr, 6));
    println!("FizzBuzz      > {}", fizzbuzz(45, vec![]));
    println!("Fibonacci     > {}", fibonacci(90));
    println!(
        "Grid traveler > {}",
        grid_traveler_memoization(20, 32, &mut HashMap::new())
    );
    println!(
        "Can sum       > {}",
        can_sum_memoization(23, &vec![10, 8, 4, 1, 32], &mut HashMap::new())
    );
}
