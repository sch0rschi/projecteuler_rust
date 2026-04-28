use itertools::Itertools;
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0093, 1258);
}

fn solve_0093() -> u32 {
    let mut max_n = 0;
    let mut max_digits: Vec<i32> = Vec::new();
    for numbers in (1..=9).combinations(4) {
        let numbers_casted = numbers.iter().map(|&n| n as f32).collect_vec();
        let n = find_consecutive_n(&numbers_casted);
        if n > max_n {
            max_digits = numbers;
            max_n = n;
        }
    }

    max_digits[0] as u32 * 1000
        + max_digits[1] as u32 * 100
        + max_digits[2] as u32 * 10
        + max_digits[3] as u32
}

fn find_consecutive_n(digits: &[f32]) -> usize {
    let mut covered = vec![false; 10_000];
    covered.fill(false);
    covered[0] = true;

    let all_results = digits
        .iter()
        .permutations(4)
        .flat_map(|permutation| {
            let digit_0: Vec<f32> = vec![*permutation[0]];
            let digit_1 = vec![*permutation[1]];
            let digit_2 = vec![*permutation[2]];
            let digit_3 = vec![*permutation[3]];

            let result_0 = apply_operation(&digit_0, &digit_1);
            let result_1 = apply_operation(&result_0, &digit_2);
            let result_chain = apply_operation(&result_1, &digit_3);
            let left = apply_operation(&digit_0, &digit_1);
            let right = apply_operation(&digit_2, &digit_3);
            let result_split = apply_operation(&left, &right);
            [result_chain, result_split].concat()
        })
        .collect_vec();

    for result in all_results {
        let rounded = result.round();
        if (result - rounded).abs() < 0.000000000000001 {
            covered[rounded as usize] = true;
        }
    }

    covered.iter().position(|&b| !b).unwrap() - 1
}

fn apply_operation(numbers_1: &Vec<f32>, numbers_2: &Vec<f32>) -> Vec<f32> {
    let mut results = Vec::new();
    for number_1 in numbers_1 {
        for number_2 in numbers_2 {
            results.push(number_1 + number_2);
            results.push(number_1 - number_2);
            results.push(number_1 * number_2);
            if *number_2 != 0.0 {
                let x = number_1 / number_2;
                results.push(x);
            }

            results.push(number_2 + number_1);
            results.push(number_2 - number_1);
            results.push(number_2 * number_1);
            if *number_1 != 0.0 {
                let x = number_2 / number_1;
                results.push(x);
            }
        }
    }
    results
}
