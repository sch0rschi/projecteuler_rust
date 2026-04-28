use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::permutations::PermutationPruner;
use std::str::FromStr;

fn main() {
    solve_print_and_check(solve_0068, 6531031914842725);
}

fn solve_0068() -> u64 {
    let mut max_16_digit_concatenation = 0;
    let elements: &[u8] = &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let mut permutation = PermutationPruner::new(elements);

    while let Some(configuration) = permutation.next_permutation() {
        if *configuration[0] > 6 {
            permutation.prune(0);
            continue;
        }
        if configuration[0] > configuration[1] {
            permutation.prune(1);
            continue;
        }
        if configuration[0] > configuration[2] {
            permutation.prune(2);
            continue;
        }
        if configuration[0] > configuration[3] {
            permutation.prune(3);
            continue;
        }
        if configuration[0] > configuration[4]
            || *configuration[5] == 10
            || *configuration[6] == 10
            || *configuration[7] == 10
            || *configuration[8] == 10
            || *configuration[9] == 10
        {
            permutation.prune(4);
            continue;
        }
        let sum_0 = configuration[0] + configuration[5] + configuration[6];
        let sum_1 = configuration[1] + configuration[6] + configuration[7];
        let sum_2 = configuration[2] + configuration[7] + configuration[8];
        let sum_3 = configuration[3] + configuration[8] + configuration[9];
        let sum_4 = configuration[4] + configuration[9] + configuration[5];

        if sum_0 != sum_1 {
            permutation.prune(7);
            continue;
        }
        if sum_0 != sum_2 {
            permutation.prune(8);
            continue;
        }
        if sum_0 != sum_3 {
            continue;
        }
        if sum_0 != sum_4 {
            continue;
        }
        let concatenation = format!(
            "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
            configuration[0],
            configuration[5],
            configuration[6],
            configuration[1],
            configuration[6],
            configuration[7],
            configuration[2],
            configuration[7],
            configuration[8],
            configuration[3],
            configuration[8],
            configuration[9],
            configuration[4],
            configuration[9],
            configuration[5],
        );
        let potential_new_max = u64::from_str(&concatenation).expect("Invalid concatenation");
        max_16_digit_concatenation = max_16_digit_concatenation.max(potential_new_max);
    }

    max_16_digit_concatenation
}
