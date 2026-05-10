const POKER_HANDS: &str = include_str!("../../resources/0054_poker.txt");
const HAND_RANKS_BYTES: &[u8] = include_bytes!("../../resources/HandRanks.dat");

pub fn solve_0054() -> i32 {
    let lookup_table = load_table();

    POKER_HANDS
        .lines()
        .filter(|line| {
            let cards = parse_line(line.as_bytes());
            let hand_1_rank = eval_5(
                lookup_table,
                [cards[0], cards[1], cards[2], cards[3], cards[4]],
            );
            let hand_2_rank = eval_5(
                lookup_table,
                [cards[5], cards[6], cards[7], cards[8], cards[9]],
            );
            hand_1_rank > hand_2_rank
        })
        .count() as i32
}

fn parse_line(b: &[u8]) -> [u8; 10] {
    std::array::from_fn(|i| encode_card(&b[i * 3..i * 3 + 2]))
}

fn encode_card(card: &[u8]) -> u8 {
    let rank = match card[0] {
        b'2'..=b'9' => card[0] - b'2',
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    };
    let suit = match card[1] {
        b'C' => 0,
        b'D' => 1,
        b'H' => 2,
        b'S' => 3,
        _ => unreachable!(),
    };
    rank * 4 + suit + 1
}

fn load_table() -> &'static [u32] {
    bytemuck::cast_slice(HAND_RANKS_BYTES)
}

fn eval_5(lookup_table: &[u32], cards: [u8; 5]) -> u32 {
    let mut p = 53usize;
    p = lookup_table[p + cards[0] as usize] as usize;
    p = lookup_table[p + cards[1] as usize] as usize;
    p = lookup_table[p + cards[2] as usize] as usize;
    p = lookup_table[p + cards[3] as usize] as usize;
    p = lookup_table[p + cards[4] as usize] as usize;
    lookup_table[p]
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0054::solve_0054;

    #[test]
    fn test() {
        solve_print_and_check(solve_0054, 376);
    }
}
