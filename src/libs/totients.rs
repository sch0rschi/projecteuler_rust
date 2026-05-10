pub fn get_totient_sieve(limit: usize) -> Vec<u32> {
    let mut phi = vec![0u32; limit + 1];
    let mut primes = Vec::with_capacity(limit / (limit as f64 / 0.9).ln().floor() as usize);
    let mut is_comp = vec![false; limit + 1];

    phi[0] = 0;
    phi[1] = 1;

    for i in 2..=limit {
        if !is_comp[i] {
            primes.push(i);
            phi[i] = i as u32 - 1;
        }

        for &p in &primes {
            let ip = i * p;
            if ip > limit {
                break;
            }

            is_comp[ip] = true;

            if i.is_multiple_of(p) {
                phi[ip] = phi[i] * p as u32;
                break;
            } else {
                phi[ip] = phi[i] * (p as u32 - 1);
            }
        }
    }

    phi
}