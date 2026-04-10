use itertools::Itertools;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0032();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(45228, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0032() -> i64 {
    let mut products: HashSet<i64> = HashSet::new();
    let permutation = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for perm in permutation.iter().cloned().permutations(permutation.len()) {
        for product_length in 1..9 {
            for multiplier_1_length in 1..(9 - product_length) {
                let (product, multiplier_1, multiplier_2) = split_array(&perm, product_length, multiplier_1_length);
                if product == multiplier_1 * multiplier_2 {
                    products.insert(product);
                }
            }
        }
    }
    products.iter().sum::<i64>()
}

fn split_array(permutation: &[i64], length_product: usize, length_multiplier_1: usize) -> (i64, i64, i64) {
    let mut product: i64 = 0;
    for digit in permutation.iter().take(length_product) {
        product *= 10;
        product += digit;
    }
    let mut multiplier_1: i64 = 0;
    for digit in permutation.iter().skip(length_product).take(length_multiplier_1) {
        multiplier_1 *= 10;
        multiplier_1 += digit;
    }
    let mut multiplier_2: i64 = 0;
    for digit in permutation.iter().skip(length_product + length_multiplier_1) {
        multiplier_2 *= 10;
        multiplier_2 += digit;
    }

    (product, multiplier_1, multiplier_2)
}