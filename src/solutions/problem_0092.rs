use bitvec::bitvec;

const LIMIT: usize = 10_000_000;


pub fn solve_0092() -> i32 {
    let mut count = 0;
    let mut seen_1 = bitvec![0; 568]; // 7 * 81 + 1
    let mut seen_89 = bitvec![0; 568];
    let mut chain: Vec<usize> = Vec::new();
    for mut i in 1..=LIMIT {
        chain.clear();
        i = next(i);
        chain.push(i);
        loop {
            if i == 1 || seen_1[i] {
                for &in_1 in &chain {
                    seen_1.set(in_1, true);
                }
                break;
            }
            if i == 89 || seen_89[i] {
                for &in_89 in &chain {
                    seen_89.set(in_89, true);
                }
                count += 1;
                break;
            }
            chain.push(i);
            i = next(i);
        }
    }
    count
}

fn next(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10) * (n % 10);
        n /= 10;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0092::solve_0092;

    #[test]
    fn test() {
        solve_print_and_check(solve_0092, 8581146);
    }
}
