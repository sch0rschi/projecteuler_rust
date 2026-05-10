use crate::libs::primes::Primes;


pub fn solve_0060() -> u64 {
    let primes = Primes::primes_inclusive(10_000);
    let primes_list = &primes.primes_list;

    let n = primes_list.len();

    let mut check = vec![vec![false; n]; n];

    for i in 0..n {
        for j in i + 1..n {
            if both_prime_concat(primes_list[i], primes_list[j], &primes) {
                check[i][j] = true;
            }
        }
    }

    let mut best = u64::MAX;

    for p0i in 0..n {
        let p0 = primes_list[p0i];
        if 5 * p0 >= best {
            break;
        }

        for p1i in p0i + 1..n {
            let p1 = primes_list[p1i];
            if p0 + 4 * p1 >= best {
                break;
            }
            if !check[p0i][p1i] {
                continue;
            }

            for p2i in p1i + 1..n {
                let p2 = primes_list[p2i];
                if p0 + p1 + 3 * p2 >= best {
                    break;
                }
                if !check[p0i][p2i] || !check[p1i][p2i] {
                    continue;
                }

                for p3i in p2i + 1..n {
                    let p3 = primes_list[p3i];
                    if p0 + p1 + p2 + 2 * p3 >= best {
                        break;
                    }
                    if !check[p0i][p3i] || !check[p1i][p3i] || !check[p2i][p3i] {
                        continue;
                    }

                    for p4i in p3i + 1..n {
                        let p4 = primes_list[p4i];
                        let sum = p0 + p1 + p2 + p3 + p4;

                        if sum >= best {
                            break;
                        }

                        if !check[p0i][p4i]
                            || !check[p1i][p4i]
                            || !check[p2i][p4i]
                            || !check[p3i][p4i]
                        {
                            continue;
                        }

                        best = sum;
                    }
                }
            }
        }
    }

    best
}

#[inline(always)]
fn both_prime_concat(p1: u64, p2: u64, primes: &Primes) -> bool {
    prime_concat(p1, p2, primes) && prime_concat(p2, p1, primes)
}

#[inline(always)]
fn prime_concat(prime_least: u64, prime_most: u64, primes: &Primes) -> bool {
    let mut factor = 10;
    while factor <= prime_least {
        factor *= 10;
    }

    primes.is_prime(prime_most * factor + prime_least)
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0060::solve_0060;

    #[test]
    fn test() {
        solve_print_and_check(solve_0060, 26033);
    }
}
