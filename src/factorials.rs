pub fn get_factorial_array(n: i64) -> Vec<i64> {
    let mut number_of_permutations: Vec<i64> = (0..=n).collect();
    number_of_permutations[0] = 1;
    for i in 1..number_of_permutations.len() {
        number_of_permutations[i] *= number_of_permutations[i - 1usize];
    }
    number_of_permutations
}
