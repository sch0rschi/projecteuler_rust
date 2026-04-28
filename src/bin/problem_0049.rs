use projecteuler::evaluation_helper::solve_print_and_check;
use projecteuler::primes::Primes;
use std::collections::HashMap;

const fn powers_of_4() -> [i32; 10] {
    let mut arr = [1; 10];
    let mut i = 1;
    while i < 10 {
        arr[i] = arr[i - 1] << 2;
        i += 1;
    }
    arr
}

const FACTORS: [i32; 10] = powers_of_4();

fn main() {
    solve_print_and_check(solve_0049, "296962999629".to_string());
}

fn solve_0049() -> String {
    let primes = Primes::primes_inclusive(9999);
    let primes_list = &primes.primes_list;
    let mut digit_count_to_primes: HashMap<u32, Vec<u64>> = HashMap::new();
    let mut prime_to_digit_counts: Vec<u32> = vec![0; 10000];
    for &prime in primes_list.iter().filter(|&&p| p > 1000) {
        let digit_count = get_digit_counts(prime);
        digit_count_to_primes
            .entry(digit_count)
            .or_default()
            .push(prime);
        prime_to_digit_counts[prime as usize] = digit_count;
    }

    let digit_count_to_primes = digit_count_to_primes;
    let prime_to_digit_counts = prime_to_digit_counts;

    for (digit_count, primes_for_digit_count) in digit_count_to_primes {
        if primes_for_digit_count.len() < 3 {
            continue;
        }
        for i in 0..primes_for_digit_count.len() {
            let lower = primes_for_digit_count[i];
            for &middle in primes_for_digit_count.iter().skip(i + 1) {
                let upper = ((middle << 1) - lower) as usize;
                if upper > 10000 {
                    break;
                } else if primes.is_prime(upper as u64)
                    && digit_count == prime_to_digit_counts[upper]
                    && lower != 1487
                    && middle != 4817
                {
                    return format!("{}{}{}", lower, middle, upper);
                }
            }
        }
    }
    unreachable!()
}

fn get_digit_counts(mut n: u64) -> u32 {
    let mut count = 0;
    while n > 0 {
        count += FACTORS[(n % 10) as usize] as u32;
        n /= 10;
    }
    count
}
