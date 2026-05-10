use bitvec::prelude::*;

struct Stage {
    multiple: u64,
    shift_10: u64,
    distinct_digits: usize,
    first: bool,
    next: Option<&'static Stage>,
}


pub fn solve_0043() -> u64 {
    let mut sum = 0;
    recursion(&STAGE_17, 0, &mut sum);

    sum
}

fn recursion(stage: &Stage, number: u64, sum: &mut u64) {
    for i in 0..=(999 / stage.multiple) {
        let stage_number = i * stage.multiple;
        if !stage.first && number / stage.shift_10 != stage_number % 100 {
            continue;
        }
        let new_number = number
            + if stage.first {
            stage_number
        } else {
            (stage_number / 100) * 100 * stage.shift_10
        };
        let new_number_digits = get_digits(new_number, stage.distinct_digits);
        let new_number_digit_count = new_number_digits.count_ones();
        if new_number_digit_count != stage.distinct_digits {
            continue;
        }
        match &stage.next {
            Some(next_stage) => recursion(next_stage, new_number, sum),
            None => {
                let missing: u64 = new_number_digits.first_zero().unwrap() as u64;
                *sum += new_number + missing * 1_000_000_000;
            }
        }
    }
}

#[inline(always)]
fn get_digits(n: u64, length: usize) -> BitVec {
    let mut digits = bitvec![0; 10];
    let mut number = n;
    for _ in 0..length {
        let digit: usize = (number % 10) as usize;
        digits.set(digit, true);
        number /= 10;
    }
    digits
}

const STAGE_17: Stage = Stage {
    multiple: 17,
    shift_10: 1,
    distinct_digits: 3,
    first: true,
    next: Some(&STAGE_13),
};

const STAGE_13: Stage = Stage {
    multiple: 13,
    shift_10: 10,
    distinct_digits: 4,
    first: false,
    next: Some(&STAGE_11),
};

const STAGE_11: Stage = Stage {
    multiple: 11,
    shift_10: 100,
    distinct_digits: 5,
    first: false,
    next: Some(&STAGE_7),
};
const STAGE_7: Stage = Stage {
    multiple: 7,
    shift_10: 1000,
    distinct_digits: 6,
    first: false,
    next: Some(&STAGE_5),
};
const STAGE_5: Stage = Stage {
    multiple: 5,
    shift_10: 10000,
    distinct_digits: 7,
    first: false,
    next: Some(&STAGE_3),
};

const STAGE_3: Stage = Stage {
    multiple: 3,
    shift_10: 100000,
    distinct_digits: 8,
    first: false,
    next: Some(&STAGE_2),
};
const STAGE_2: Stage = Stage {
    multiple: 2,
    shift_10: 1000000,
    distinct_digits: 9,
    first: false,
    next: None,
};

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0043::solve_0043;

    #[test]
    fn test() {
        solve_print_and_check(solve_0043, 16695334890);
    }
}
