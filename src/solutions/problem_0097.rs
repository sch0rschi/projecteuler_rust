use crate::libs::exponentiation::mod_pow;

const EXPONENT: usize = 7_830_457;
const FACTOR: usize = 28_433;
const MOD: usize = 10_000_000_000;


pub fn solve_0097() -> usize {
    (FACTOR * mod_pow(2, EXPONENT, MOD) + 1) % MOD
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0097::solve_0097;

    #[test]
    fn test() {
        solve_print_and_check(solve_0097, 8739992577);
    }
}
