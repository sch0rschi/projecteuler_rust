pub fn phi(n: u64, prime_factors: &[u64]) -> u64 {
    let mut result = n;
    for &p in prime_factors {
        result -= result / p;
    }
    result
}
