use projecteuler::primes::{primes_inclusive};
use std::time::Instant;

const POW10: [u64; 10] = [
    1,
    10,
    100,
    1_000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
];

type PosArray = [usize; 7]; // pos[0] = len, pos[1..] = positions

fn main() {
    let start = Instant::now();
    let result = solve_0051();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(121313, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0051() -> u64 {
    let primes = primes_inclusive(999_999);
    let primes_list = &primes.primes_list;

    let mut positions: [PosArray; 3] = [[0; 7]; 3];

    for &prime in primes_list {
        if prime < 100_000 {
            continue;
        }
        fill_relevant_digit_positions(prime, &mut positions);

        for (digit, pos_arr) in positions.iter().enumerate() {
            let len = pos_arr[0];
            if len < 3 {
                continue;
            }

            for i in 0..len - 2 {
                for j in i + 1..len - 1 {
                    for k in j + 1..len {
                        let mask = POW10[pos_arr[i + 1]] + POW10[pos_arr[j + 1]] + POW10[pos_arr[k + 1]];
                        let base = prime - digit as u64 * mask;

                        let mut count = 0;
                        let mut remaining = 10;

                        for replacement_digit in 0..=9 {
                            remaining -= 1;
                            let candidate = base + replacement_digit * mask;

                            if candidate > 100_000 && primes.is_prime(candidate) {
                                count += 1;
                            }

                            if count + remaining < 8 {
                                break;
                            }
                        }

                        if count >= 8 {
                            for d in 0..=9 {
                                let candidate = base + d * mask;
                                if primes.is_prime(candidate) {
                                    return candidate;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    unreachable!()
}

fn fill_relevant_digit_positions(mut n: u64, positions: &mut [PosArray; 3]) {
    positions[0][0] = 0;
    positions[1][0] = 0;
    positions[2][0] = 0;

    let mut pos = 1;
    n /= 10;

    while n > 0 {
        let digit = (n % 10) as usize;
        if digit < 3 {
            let len = positions[digit][0];
            if len < 6 { // maximum 6 positions
                positions[digit][len + 1] = pos;
                positions[digit][0] = len + 1;
            }
        }
        n /= 10;
        pos += 1;
    }
}
