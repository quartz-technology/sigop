mod optimizer;

use crate::optimizer::{build_combinations, find_optimization};

fn main() {
    let suffix_length = 4;
    let combinations = build_combinations(suffix_length);

    find_optimization("myFunction_", "(address)", &combinations, 3);
    println!("Hello, world!");
}
