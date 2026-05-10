use itertools::Itertools;


pub fn solve_0090() -> usize {
    let dice_source = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut valid_dice_set: usize = 0;

    let mut dice_1: [u8; 6] = [0; 6];

    for mut combination_1 in dice_source.iter().combinations(6) {
        set_dice_values(&mut dice_1, &combination_1);
        combination_1.sort();
        for combination_2 in dice_source.iter().combinations(6) {
            let mut dice_2: [u8; 6] = [0; 6];
            set_dice_values(&mut dice_2, &combination_2);
            dice_2.sort();
            if dice_1 > dice_2 {
                continue;
            }
            if check_dice(&dice_1, &dice_2) {
                valid_dice_set += 1;
            }
        }
    }

    valid_dice_set
}

fn set_dice_values(dice_1: &mut [u8; 6], combination_1: &Vec<&u8>) {
    for (i, &&val) in combination_1.iter().enumerate() {
        dice_1[i] = if val != 9 { val } else { 6 };
    }
}

fn check_dice(dice_1: &[u8; 6], dice_2: &[u8; 6]) -> bool {
    check_dice_combination(dice_1, dice_2, 0, 1)
        && check_dice_combination(dice_1, dice_2, 0, 4)
        && check_dice_combination(dice_1, dice_2, 0, 6)
        && check_dice_combination(dice_1, dice_2, 1, 6)
        && check_dice_combination(dice_1, dice_2, 2, 5)
        && check_dice_combination(dice_1, dice_2, 3, 6)
        && check_dice_combination(dice_1, dice_2, 4, 6)
        && check_dice_combination(dice_1, dice_2, 8, 1)
}

fn check_dice_combination(dice_1: &[u8; 6], dice_2: &[u8; 6], digit_1: u8, digit_2: u8) -> bool {
    dice_1.contains(&digit_1) && dice_2.contains(&digit_2)
        || dice_1.contains(&digit_2) && dice_2.contains(&digit_1)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0090::solve_0090;

    #[test]
    fn test() {
        solve_print_and_check(solve_0090, 1217);
    }
}
