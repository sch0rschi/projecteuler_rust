pub struct ProperDivisorSums {
    pub divisor_sums: Vec<u32>,
}

impl ProperDivisorSums {
    pub fn new(limit: usize) -> Self {
        let mut smallest_prime_factors: Vec<u32> = (0..=limit as u32).collect();
        let mut i = 2usize;
        while i * i <= limit {
            if smallest_prime_factors[i] == i as u32 {
                for j in (i * i..=limit).step_by(i) {
                    if smallest_prime_factors[j] == j as u32 {
                        smallest_prime_factors[j] = i as u32;
                    }
                }
            }
            i += 1;
        }

        let mut sigma: Vec<u32> = vec![0; limit + 1];
        sigma[1] = 1;
        for n in 2..=limit {
            let p = smallest_prime_factors[n] as usize;
            let m = n / p;
            if smallest_prime_factors[m] != p as u32 {
                sigma[n] = (1 + p as u32) * sigma[m];
            } else {
                sigma[n] = sigma[m] * (1 + p as u32) - sigma[m / p] * p as u32;
            }
        }

        let divisor_sums = sigma.iter().enumerate()
            .map(|(n, &s)| s.saturating_sub(n as u32))
            .collect();

        ProperDivisorSums { divisor_sums }
    }

    pub fn get(self: &ProperDivisorSums, n: u32) -> u32 {
        self.divisor_sums[n as usize]
    }
}


