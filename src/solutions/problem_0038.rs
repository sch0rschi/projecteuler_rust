pub fn solve_0038() -> u64 {
    (1u64..10_000)
        .filter_map(get_max_pandigital)
        .max()
        .unwrap()
}

fn get_max_pandigital(i: u64) -> Option<u64> {
    let mut value = 0u64;
    let mut used = 0u16;
    let mut digits = 0u32;

    for multiplier in 1u64..=9 {
        let n = i * multiplier;

        let mut pow10 = 1u64;
        if n >= 10000 {
            pow10 = 10000;
        } else if n >= 1000 {
            pow10 = 1000;
        } else if n >= 100 {
            pow10 = 100;
        } else if n >= 10 {
            pow10 = 10;
        }

        let mut x = n;
        let mut p = pow10;

        while p > 0 {
            let digit = (x / p) as u16;
            x %= p;
            p /= 10;

            if digit == 0 {
                return None;
            }

            let bit = 1 << digit;
            if used & bit != 0 {
                return None;
            }

            used |= bit;
            digits += 1;

            value = value * 10 + digit as u64;
        }

        if digits == 9 {
            return if used == 0b1111111110 {
                Some(value)
            } else {
                None
            };
        }

        if digits > 9 {
            return None;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0038::solve_0038;

    #[test]
    fn test() {
        solve_print_and_check(solve_0038, 932718654);
    }
}
