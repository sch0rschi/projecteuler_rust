use std::time::Instant;

fn main() {
    let start = Instant::now();

    for d_i in 1i64.. {
        let d = p(d_i);

        for k in 2.. {
            let p_k = p(k);

            if p_k <= d {
                continue;
            }

            let candidate = p_k - d;

            if is_scaled_pentagonal(candidate) {
                if is_scaled_pentagonal(p_k + candidate) {
                    println!("{}", d / 2);
                    println!("Elapsed: {:?}", start.elapsed());
                    return;
                }
            }

            if p_k - p(k - 1) > d {
                break;
            }
        }
    }
}

fn p(i: i64) -> i64 {
    i * (3 * i - 1)
}

fn is_scaled_pentagonal(x: i64) -> bool {
    let d = 1.0 + 12.0 * x as f64;
    let sqrt_d = d.sqrt();

    sqrt_d.fract() == 0.0 && (1.0 + sqrt_d) % 6.0 == 0.0
}
