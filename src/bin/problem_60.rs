use projecteuler::primes::{primes_inclusive, Primes};
use std::time::Instant;
use num_integer::Roots;

fn main() {
    let start = Instant::now();

    let Primes {
        prime_sieve,
        mut prime_list,
    } = primes_inclusive(100_000_000);

    let last_checkable = *prime_list.last().unwrap();
    prime_list.retain(|x| x < &last_checkable.sqrt());

    let mut check: [[bool; 1500]; 1500] = [[false; 1500]; 1500];

    for i in 0..prime_list.len() {
        for j in i + 1..prime_list.len() {
            if both_prime_concat(prime_list[i], prime_list[j], &prime_sieve) {
                check[i][j] = true;
                check[j][i] = true;
            }
        }
    }

    let mut smallest_sum: u64 = u64::MAX;

    for p0i in 0..prime_list.len() {
        if 5 * prime_list[p0i] > smallest_sum {
            break;
        }
        for p1i in p0i + 1..prime_list.len() {
            if prime_list[p0i] + 4 * prime_list[p1i] > smallest_sum {
                break;
            }
            if !check[p0i][p1i] {
                continue;
            }
            for p2i in p1i + 1..prime_list.len() {
                if prime_list[p0i] + prime_list[p1i] + 3 * prime_list[p2i] > smallest_sum {
                    break;
                }
                if !check[p0i][p2i] || !check[p1i][p2i] {
                    continue;
                }
                for p3i in p2i + 1..prime_list.len() {
                    if prime_list[p0i] + prime_list[p1i] + prime_list[p2i] + 2 * prime_list[p3i] > smallest_sum {
                        break;
                    }
                    if !check[p0i][p3i] || !check[p1i][p3i] || !check[p2i][p3i] {
                        continue;
                    }
                    for p4i in p3i + 1..prime_list.len() {
                        if prime_list[p0i] + prime_list[p1i] + prime_list[p2i] + prime_list[p3i] + prime_list[p4i] > smallest_sum {
                            break;
                        }
                        if !check[p0i][p4i] || !check[p1i][p4i] || !check[p2i][p4i] || !check[p3i][p4i] {
                            continue;
                        }
                        smallest_sum = prime_list[p0i] + prime_list[p1i] + prime_list[p2i] + prime_list[p3i] + prime_list[p4i];
                    }
                }
            }
        }
    }


    println!("{}", smallest_sum);
    println!("Elapsed: {:?}", start.elapsed());
}

fn both_prime_concat(p1: u64, p2: u64, prime_sieve: &[bool]) -> bool {
    prime_concat(p1, p2, prime_sieve) && prime_concat(p2, p1, prime_sieve)
}

fn prime_concat(prime_least: u64, prime_most: u64, prime_sieve: &[bool]) -> bool {
    let mut multiplyer = 10;
    while multiplyer < prime_least {
        multiplyer *= 10;
    }
    prime_sieve[(prime_most * multiplyer + prime_least) as usize]
}
