fn main() {
    let mut permutation = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let permutation_len = permutation.len();
    let number_of_possible_permutations = create_factorial_array();
    let mut n_th_permutation = 1_000_000 - 1;
    while n_th_permutation > 0 {
        let find = number_of_possible_permutations.binary_search(&n_th_permutation);
        let find_sanitized = find.unwrap_or_else(|i| i - 1);
        let number_of_possible_permutations = number_of_possible_permutations[find_sanitized];
        let offset = n_th_permutation / number_of_possible_permutations;
        let decrease = (offset) * number_of_possible_permutations;
        n_th_permutation -= decrease;
        let swap_index = permutation_len - 2 - find_sanitized;
        permutation.swap(swap_index, swap_index + offset);
        permutation[swap_index + 1..].sort();
    }
    println!("{}", permutation.iter().map(|x| x.to_string()).collect::<String>());
}

fn create_factorial_array() -> [usize; 10] {
    let mut number_of_permutations = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in 1..number_of_permutations.len() {
        number_of_permutations[i] *= number_of_permutations[i - 1usize];
    }
    number_of_permutations
}
