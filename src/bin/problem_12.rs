fn main() {
    let mut triangle_number = 0;
    for i in 1.. {
        triangle_number += i;
        let divisor_count = count_divisors(triangle_number);
        if divisor_count > 500 {
            println!("{}", triangle_number);
            break;
        }
    }
}

fn count_divisors(n: i32) -> i32 {
    let mut n_sqrt = n.isqrt();
    let mut divisor_count = 0;
    if n_sqrt * n_sqrt == n {
        divisor_count = 1;
    } else {
        n_sqrt += 1;
    }

    for divisor_candidate in 1..n_sqrt {
        if n % divisor_candidate == 0 {
            divisor_count += 2;
        }
    }
    divisor_count
}
