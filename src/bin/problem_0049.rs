use projecteuler::primes::{primes_inclusive};
use std::collections::HashMap;
use std::time::Instant;

const fn powers_of_4() -> [i32; 10] {
    let mut arr = [1; 10];
    let mut i = 1;
    while i < 10 {
        arr[i] = arr[i - 1] << 2;
        i += 1;
    }
    arr
}

static FACTORS: [i32; 10] = powers_of_4();

fn main() {
    let start = Instant::now();
    let result = solve_0049();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!("296962999629", result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0049() -> String {
    let primes = primes_inclusive(9999);
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
