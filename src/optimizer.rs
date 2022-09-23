use itertools::Itertools;
use rayon::prelude::*;
use sha3::{Digest, Keccak256};

/// Creates a vector containing all possible combinations of elements in a dictionary, stopping when
/// the elements length reach the provided suffix_length variable.
pub fn build_combinations(suffix_length: u8) -> Vec<String> {
    let dictionary = vec![
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h",
        "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z", "$", "_",
    ];

    let mut combinations: Vec<String> = dictionary.iter().map(|&s| s.into()).collect();

    for n in 1..suffix_length {
        let mut new_combinations: Vec<String> = (1..n).fold(
            dictionary
                .iter()
                .cartesian_product(dictionary.iter())
                .map(|(&a, &b)| a.to_owned() + b)
                .collect(),
            |acc, _| {
                acc.into_iter()
                    .cartesian_product(dictionary.iter())
                    .map(|(a, b)| a + b)
                    .collect()
            },
        );
        combinations.append(&mut new_combinations);
    }

    combinations
}

/// Finds a way to name the initial function that will produce a function signature with a specific
/// count (the optimization_target) of leading zeros.
/// Using the combinations, reconstruct a function name by emplacing each one of them right before
/// the function parameters, computes the new function signature and verifies if it match the target.
pub fn find_optimization(
    initial_function_name: &str,
    initial_function_parameters: &str,
    combinations: &Vec<String>,
    optimization_target: u8,
) {
    // We use rayon to iterate in the combination in parallel.
    let suffix = combinations.par_iter().find_first(|&x| {
        // Reconstructs the function's signature using the current combination.
        let mut new_function_signature = String::from(initial_function_name);
        new_function_signature.push_str(x);
        new_function_signature.push_str(initial_function_parameters);

        // Encodes the new function signature.
        let data = new_function_signature.into_bytes();
        let mut encoded_new_function_signature = [0u8; 4];
        encoded_new_function_signature.copy_from_slice(&Keccak256::digest(&data)[..4]);

        // Verifies if the new encoded function signature matches the optimization target.
        let mut found = true;
        for i in 0..optimization_target {
            if encoded_new_function_signature[i as usize] != 0 {
                found = false;
                break;
            }
        }

        found
    });

    println!("{:?}", suffix.unwrap())
}

/*
fn compute_maximum_number_of_combinations(dictionary_length: u64, suffix_length: u8) -> u64 {
    (1..=suffix_length)
        .into_iter()
        .map(|n| dictionary_length.pow(n as u32) as u64)
        .sum()
}

fn compute_total_bytes_used(dictionary_length: u64, suffix_length: u8) -> u64 {
    (1..=suffix_length)
        .into_iter()
        .map(|n| dictionary_length.pow(n as u32) as u64 * (n as u64))
        .sum()
}
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_finds_an_optimization() {}
}
