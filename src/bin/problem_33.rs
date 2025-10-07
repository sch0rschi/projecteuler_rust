use num_integer::Integer;
use projecteuler::digits::get_digits;

fn main() {
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
    println!("{}", denominator_product / gcd);
}
