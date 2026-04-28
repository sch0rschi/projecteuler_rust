use num_integer::Integer;
use projecteuler::digits::get_digits;
use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0033, 100);
}

fn solve_0033() -> u64 {
    let mut nominator_product = 1;
    let mut denominator_product = 1;
    for nominator in 10..100 {
        for denominator in nominator + 1..100 {
            let nominator_digits = get_digits(nominator);
            let denominator_digits = get_digits(denominator);
            if nominator_digits[0] != nominator_digits[1]
                && denominator_digits[0] != denominator_digits[1]
            {
                if nominator_digits[0] == denominator_digits[0]
                    && nominator_digits[0] != 0
                    && nominator * denominator_digits[1] == nominator_digits[1] * denominator
                {
                    nominator_product *= nominator;
                    denominator_product *= denominator;
                }
                if nominator_digits[1] == denominator_digits[0]
                    && nominator * denominator_digits[1] == nominator_digits[0] * denominator
                {
                    nominator_product *= nominator;
                    denominator_product *= denominator;
                }
                if nominator_digits[0] == denominator_digits[1]
                    && nominator * denominator_digits[0] == nominator_digits[1] * denominator
                {
                    nominator_product *= nominator;
                    denominator_product *= denominator;
                }
                if nominator_digits[1] == denominator_digits[1]
                    && nominator * denominator_digits[0] == nominator_digits[1] * denominator
                {
                    nominator_product *= nominator;
                    denominator_product *= denominator;
                }
            }
        }
    }
    let gcd = nominator_product.gcd(&denominator_product);
    denominator_product / gcd
}
