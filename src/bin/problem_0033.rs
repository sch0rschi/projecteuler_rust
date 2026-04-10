use num_integer::Integer;
use projecteuler::digits::get_digits;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0033();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(100, result);
    assert!(duration < std::time::Duration::from_secs(1));
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
