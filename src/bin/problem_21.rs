use projecteuler::divisors::proper_divisor_sum;

fn main() {
    let mut d: [u64; 10_000] = [0; 10_000];
    let mut amicable_numbers_sum = 0;
    for i in 1..10_000 {
        let proper_divisor_sum = proper_divisor_sum(i);
        d[i as usize] = proper_divisor_sum;
        if proper_divisor_sum < i && d[proper_divisor_sum as usize] == i {
            amicable_numbers_sum += i;
            amicable_numbers_sum += proper_divisor_sum;
        }
    }
    println!("{}", amicable_numbers_sum);
}
