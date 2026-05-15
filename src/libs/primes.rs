use std::iter::repeat_n;
use primal::Sieve;

pub struct Primes {
    sieve: Sieve,
    limit: usize,
}

impl Primes {
    pub fn primes_inclusive(limit: usize) -> Self {
        Self {
            sieve: Sieve::new(limit),
            limit,
        }
    }

    pub fn is_prime(&self, n: usize) -> bool {
        if self.sieve.upper_bound() < n {
            primal::is_prime(n as u64)
        } else {
            self.sieve.is_prime(n)
        }
    }

    pub fn unique_prime_factors(&self, n: usize) -> Vec<usize> {
        self.sieve
            .factor(n)
            .unwrap_or_default()
            .into_iter()
            .map(|(p, _)| p)
            .collect()
    }

    pub fn prime_factors(&self, n: usize) -> Vec<usize> {
        self.sieve
            .factor(n)
            .unwrap_or_default()
            .into_iter()
            .flat_map(|(p, exp)| repeat_n(p, exp))
            .collect()
    }

    pub fn get_primes_list(&self) -> Vec<usize> {
        self.sieve
            .primes_from(2)
            .take_while(|&p| p <= self.limit)
            .collect()
    }

    pub fn single_iterator(&self) -> impl Iterator<Item = usize> + '_ {
        self.sieve.primes_from(2)
    }
}
