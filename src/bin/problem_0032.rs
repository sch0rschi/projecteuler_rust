use num_integer::Integer;
use projecteuler::evaluation_helper::solve_print_and_check;

const FULL: u16 = 0b11_1111_1110;
const PRODUCT_LIMIT: usize = 10_000;

#[derive(Copy, Clone)]
struct Digits {
    valid: bool,
    mask: u16,
}

fn main() {
    solve_print_and_check(solve_0032, 45228);
}

fn solve_0032() -> usize {
    let digits_table = build_table();

    let valid_1_digit: Vec<usize> = (1..10).filter(|&n| digits_table[n].valid).collect();
    let valid_2_digit: Vec<usize> = (12..100).filter(|&n| digits_table[n].valid).collect();
    let valid_3_digit: Vec<usize> = (123..1_000).filter(|&n| digits_table[n].valid).collect();
    let valid_4_digit: Vec<usize> = (1234..10_000).filter(|&n| digits_table[n].valid).collect();

    let mut is_added = [false; PRODUCT_LIMIT + 1];
    let mut sum = 0;

    sum_pandigital_products(
        &digits_table,
        &mut is_added,
        &mut sum,
        &valid_1_digit,
        &valid_4_digit,
    );

    sum_pandigital_products(
        &digits_table,
        &mut is_added,
        &mut sum,
        &valid_2_digit,
        &valid_3_digit,
    );

    sum
}

fn sum_pandigital_products(
    digits_table: &[Digits; PRODUCT_LIMIT + 1],
    is_added: &mut [bool; PRODUCT_LIMIT + 1],
    sum: &mut usize,
    factors_1: &[usize],
    factors_2: &[usize],
) {
    for &factor_1 in factors_1 {
        let d1 = digits_table[factor_1];
        let max_factor_2 = PRODUCT_LIMIT / factor_1;

        for &factor_2 in factors_2 {
            if factor_2 >= max_factor_2 {
                break;
            }

            let d2 = digits_table[factor_2];

            if (d1.mask & d2.mask) != 0 {
                continue;
            }

            let combined = d1.mask | d2.mask;
            let remaining = FULL & !combined;
            if remaining == 0 {
                continue;
            }

            let product = factor_1 * factor_2;
            if is_added[product] {
                continue;
            }

            let dp = digits_table[product];
            if dp.valid && dp.mask == remaining {
                is_added[product] = true;
                *sum += product;
            }
        }
    }
}

fn build_table() -> [Digits; PRODUCT_LIMIT + 1] {
    let mut table = [Digits {
        valid: false,
        mask: 0,
    }; PRODUCT_LIMIT + 1];

    for (table_index, table_element) in table.iter_mut().enumerate().skip(1) {
        let mut mask = 0u16;
        let mut valid = true;
        let mut div = table_index;
        let mut rem;

        while div > 0 {
            (div, rem) = div.div_rem(&10);

            if rem == 0 {
                valid = false;
                break;
            }

            let digit_mask = 1u16 << rem;

            if (mask & digit_mask) != 0 {
                valid = false;
                break;
            }

            mask |= digit_mask;
        }

        *table_element = Digits { valid, mask };
    }

    table
}
