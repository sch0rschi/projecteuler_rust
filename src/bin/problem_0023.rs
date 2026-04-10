use bitvec::bitvec;
use projecteuler::divisors::proper_divisor_sum;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0023();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(4179871, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0023() -> usize {
    let mut abundant_numbers: Vec<u64> = Vec::new();
    for i in 2..=28123 {
        let proper_divisor_sum = proper_divisor_sum(i);
        if proper_divisor_sum > i {
            abundant_numbers.push(i);
        }
    }
    let mut bits = bitvec![1; 28123];
    for n_1 in &abundant_numbers {
        for n_2 in &abundant_numbers {
            let abundant_sum = n_1 + n_2;
            if abundant_sum < 28123 {
                bits.set(abundant_sum as usize, false);
            }
        }
    }
    let mut sum = 0;
    for (index, value) in bits.iter().enumerate() {
        if *value {
            sum += index;
        }
    }
    sum
}
