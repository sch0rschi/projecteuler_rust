use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0045();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(1533776805, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0045() -> i32 {
    // all hexagonal numbers are also triangle numbers, hence ignoring triangle numbers

    // Starting after known solution 40755:
    // P(165) = H(143) = 40755

    let mut p_j = 40755;
    let mut j = 165;

    let mut h_k = 40755;
    let mut k = 143;

    loop {
        if p_j <= h_k {
            p_j += 3 * j + 1;
            j += 1;
        } else {
            h_k += 4 * k + 1;
            k += 1;
        }

        if p_j == h_k {
            return p_j;
        }
    }
}
