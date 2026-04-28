use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::factorials::get_factorial_array;

fn main() {
    solve_print_and_check(solve_0024, "2783915460".to_string());
}

fn solve_0024() -> String {
    let mut permutation = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let permutation_len = permutation.len();
    let number_of_possible_permutations: Vec<u64> =
        get_factorial_array(10).iter().skip(1).copied().collect();
    let mut n_th_permutation = 1_000_000 - 1;
    while n_th_permutation > 0 {
        let find = number_of_possible_permutations.binary_search(&n_th_permutation);
        let find_sanitized = find.unwrap_or_else(|i| i - 1);
        let number_of_possible_permutations = number_of_possible_permutations[find_sanitized];
        let offset = n_th_permutation / number_of_possible_permutations;
        let decrease = (offset) * number_of_possible_permutations;
        n_th_permutation -= decrease;
        let swap_index = permutation_len - 2 - find_sanitized;
        permutation.swap(swap_index, swap_index + offset as usize);
        permutation[swap_index + 1..].sort();
    }
    permutation
        .iter()
        .map(|x| x.to_string())
        .collect::<String>()
}
