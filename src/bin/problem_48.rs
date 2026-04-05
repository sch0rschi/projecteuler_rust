use std::time::Instant;

static MODULO: u128 = 10_000_000_000;

fn main() {
    let start = Instant::now();
    let mut partial_powers: [u128; 10001] = [0; 10001];

    let mut sum: u128 = 0;
    for i in 1u128..1000 {
        sum += i_to_the_power_of_i_modulo(i, &mut partial_powers);
        sum %= MODULO;
    }

    println!("{}", sum);
    println!("Elapsed: {:?}", start.elapsed());
}

fn i_to_the_power_of_i_modulo(i: u128, partial_powers: &mut [u128; 10001]) -> u128 {
    let mut product = i;
    let mut power: u128 = 1;
    partial_powers[1] = i;
    while power < i {
        let remainder = i - power;
        let pow_2_remainder = 1 << remainder.ilog2();
        if pow_2_remainder <= power {
            let i1 = partial_powers[pow_2_remainder as usize];
            product *= i1;
            power += pow_2_remainder;
            product %= MODULO;
        } else {
            product *= product;
            product %= MODULO;
            power *= 2;
            partial_powers[power as usize] = product;
        }
    }
    product % MODULO
}
