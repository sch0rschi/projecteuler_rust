use std::time::Instant;

fn main() {
    let start = Instant::now();

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
            println!("{}", p_j);
            break;
        }
    }

    println!("Elapsed: {:?}", start.elapsed());
}
