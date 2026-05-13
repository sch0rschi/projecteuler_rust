pub fn get_totient_sieve(limit: usize) -> Vec<u32> {
    let mut phi = vec![0u32; limit + 1];
    let mut is_comp = vec![0u64; (limit + 64) / 64];
    let mut primes: Vec<u32> = Vec::with_capacity(estimate_prime_count(limit));

    #[inline(always)]
    fn mark_comp(is_comp: &mut [u64], n: usize) {
        is_comp[n >> 6] |= 1u64 << (n & 63);
    }

    #[inline(always)]
    fn check_comp(is_comp: &[u64], n: usize) -> bool {
        is_comp[n >> 6] & (1u64 << (n & 63)) != 0
    }

    phi[1] = 1;

    for i in 2..=limit {
        if !check_comp(&is_comp, i) {
            primes.push(i as u32);
            phi[i] = i as u32 - 1;
        }

        let phi_i = phi[i];

        for &p in primes.iter() {
            let p = p as usize;
            let ip = i * p;
            if ip > limit {
                break;
            }

            mark_comp(&mut is_comp, ip);

            if i % p == 0 {
                phi[ip] = phi_i * p as u32;
                break;
            }
            phi[ip] = phi_i * (p as u32 - 1);
        }
    }
    phi
}

#[inline]
fn estimate_prime_count(n: usize) -> usize {
    if n < 6 { return 3; }
    // Rosser's theorem: pi(n) < n/(ln(n)-1.1) for n >= 60
    (n as f64 / ((n as f64).ln() - 1.1)) as usize
}
