use itertools::Itertools;
use rand::prelude::*;
use std::time::Instant;

const SQUARES: usize = 40;
const DICE_SIDES: usize = 4;
const GO: usize = 0;
const JAIL: usize = 10;
const G2J: usize = 30;

fn main() {
    let start = Instant::now();
    let result = solve_0084();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(101524, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0084() -> u32 {
    let mut rng = rand::rng();

    let mut square = 0usize;
    let mut double_count = 0usize;
    let mut square_counter = [0u64; SQUARES];
    let mut count: u128 = 0;

    let mut cc_cards = (1usize..=16).collect_array().unwrap();
    cc_cards.shuffle(&mut rng);

    let mut ch_cards = (1usize..=16).collect_array().unwrap();
    ch_cards.shuffle(&mut rng);

    const CHECK_INTERVAL: u128 = 100_000;

    loop {
        let dice_1 = rng.random_range(1..=DICE_SIDES);
        let dice_2 = rng.random_range(1..=DICE_SIDES);

        if dice_1 == dice_2 {
            double_count += 1;
        } else {
            double_count = 0;
        }

        if double_count == 3 {
            double_count = 0;
            square = JAIL;
        } else {
            square = (square + dice_1 + dice_2) % SQUARES;

            if square == G2J {
                square = JAIL;
            }

            if square == GO {
                cc_cards.shuffle(&mut rng);
                ch_cards.shuffle(&mut rng);
            }

            square = match square {
                2 | 17 | 33 => handle_cc(square, &mut cc_cards),
                7 | 22 | 36 => handle_ch(square, &mut ch_cards),
                _ => square,
            };
        }

        square_counter[square] += 1;
        count += 1;

        if count.is_multiple_of(CHECK_INTERVAL)
            && let Some((a, b, c)) = converged(&square_counter)
        {
            return (a * 10000 + b * 100 + c) as u32;
        }
    }
}

fn handle_ch(square: usize, ch_cards: &mut [usize; 16]) -> usize {
    let ch_card = ch_cards[0];
    ch_cards.rotate_left(1);

    match ch_card {
        1 => GO,
        2 => JAIL,
        3 => 11,
        4 => 24,
        5 => 39,
        6 => 5,
        7 | 8 => next_r(square),
        9 => next_u(square),
        10 => {
            let new_square = square.saturating_sub(3);

            if 33 == new_square {
                handle_cc(new_square, ch_cards)
            } else {
                new_square
            }
        }
        _ => square,
    }
}

fn handle_cc(square: usize, cc_cards: &mut [usize; 16]) -> usize {
    let cc_card = cc_cards[0];
    cc_cards.rotate_left(1);

    match cc_card {
        1 => GO,
        2 => JAIL,
        _ => square,
    }
}

fn next_r(x: usize) -> usize {
    (((x + 5) / 10) * 10 + 5) % SQUARES
}

fn next_u(x: usize) -> usize {
    if !(12..28).contains(&x) { 12 } else { 28 }
}

fn converged(square_counter: &[u64; SQUARES]) -> Option<(usize, usize, usize)> {
    let total = square_counter.iter().sum::<u64>() as f64;
    if total == 0.0 {
        return None;
    }

    let mut top: [(usize, u64); SQUARES] = {
        let mut arr = [(0usize, 0u64); SQUARES];
        for (i, c) in square_counter.iter().enumerate() {
            arr[i] = (i, *c);
        }
        arr
    };

    top.sort_by(|a, b| b.1.cmp(&a.1));

    let top4 = &top[0..4];

    let mut stats = [(0usize, 0f64, 0f64, 0f64); 4];

    for (i, &(idx, count)) in top4.iter().enumerate() {
        let p = count as f64 / total;
        let sigma = (p * (1.0 - p) / total).sqrt();
        let margin = 1.96 * sigma;

        stats[i] = (idx, p, p - margin, p + margin);
    }

    let overlap = (0..4).any(|i| (i + 1..4).any(|j| interval_overlap(stats[i], stats[j])));

    if !overlap {
        let a = top4[0].0;
        let b = top4[1].0;
        let c = top4[2].0;
        return Some((a, b, c));
    }

    None
}

fn interval_overlap(a: (usize, f64, f64, f64), b: (usize, f64, f64, f64)) -> bool {
    let (_, _, a_low, a_high) = a;
    let (_, _, b_low, b_high) = b;

    a_low <= b_high && b_low <= a_high
}
