pub struct ProperDivisorSums {
    divisor_sums: Vec<u32>,
}

impl ProperDivisorSums {
    pub fn new(limit: usize) -> Self {
        let mut divisor_sums = vec![0; limit + 1];
        for i in 1usize..=limit / 2 {
            for j in (2 * i..=limit).step_by(i) {
                divisor_sums[j] += i as u32;
            }
        }

        ProperDivisorSums {
            divisor_sums
        }
    }

    pub fn get(self: &ProperDivisorSums, n: u32) -> u32 {
        self.divisor_sums[n as usize]
    }
}


