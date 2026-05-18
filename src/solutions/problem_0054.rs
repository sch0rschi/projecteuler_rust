const POKER_HANDS: &str = include_str!("../../resources/0054_poker.txt");

pub fn solve_0054() -> u32 {
    POKER_HANDS
        .lines()
        .filter(|line| {
            let b = line.as_bytes();
            let hand1 = parse_hand(&b[..14]);
            let hand2 = parse_hand(&b[15..]);
            eval_hand(hand1) > eval_hand(hand2)
        })
        .count() as u32
}

// One card: bits 0-3 = suit (one-hot), bits 4-7 = rank (0=2 .. 12=A)
fn parse_card(b: &[u8]) -> u8 {
    let rank = match b[0] {
        b'2'..=b'9' => b[0] - b'2',
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => unreachable!(),
    };
    let suit = match b[1] {
        b'C' => 0,
        b'D' => 1,
        b'H' => 2,
        b'S' => 3,
        _ => unreachable!(),
    };
    (rank << 2) | suit
}

fn parse_hand(b: &[u8]) -> [u8; 5] {
    std::array::from_fn(|i| parse_card(&b[i * 3..]))
}

// Returns a u32 where higher = better hand. Fully comparable across categories.
fn eval_hand(cards: [u8; 5]) -> u32 {
    // --- build suit-rows: bits [suit*13 + rank] ---
    let mut suit_rows = 0u64;
    // --- nibble counts: nibble[rank] = count ---
    let mut counts = 0u64;

    for &c in &cards {
        let rank = (c >> 2) as u64;
        let suit = (c & 3) as u64;
        suit_rows |= 1 << (suit * 13 + rank);
        counts += 1 << (rank * 4);
    }

    // ranks present (OR of all four suit rows)
    let present: u32 = ((suit_rows
        | suit_rows >> 13
        | suit_rows >> 26
        | suit_rows >> 39) & 0x1FFF) as u32;

    // --- flush: one suit row has 5 cards ---
    let flush = (0..4).find(|&s| {
        ((suit_rows >> (s * 13)) & 0x1FFF).count_ones() == 5
    });

    // --- straight: 5 consecutive ranks, including wheel (A-2-3-4-5) ---
    let straight_high = detect_straight(present);

    // --- group cards by count for category + tiebreak ---
    // Sort ranks by (count desc, rank desc) → pack into nibbles of result
    let tiebreak = tiebreak_key(counts);

    let category: u32 = match max_count(counts) {
        4 => 7, // four of a kind
        3 if has_count(counts, 2) => 6, // full house
        3 => 3, // three of a kind
        2 if two_pairs(counts) => 2, // two pair
        2 => 1, // one pair
        _ => {
            if flush.is_some() && straight_high.is_some() {
                8 // straight flush
            } else if flush.is_some() {
                5 // flush
            } else if straight_high.is_some() {
                4 // straight
            } else {
                0 // high card
            }
        }
    };

    // For straight (flush), tiebreak is just the high card
    let tiebreak = straight_high.map(|t| t as u32).unwrap_or(tiebreak);

    (category << 24) | tiebreak
}

// Returns Some(high_rank) if the 13-bit present mask is a straight, else None.
fn detect_straight(present: u32) -> Option<u8> {
    // Normal straights: 5 consecutive bits
    for high in (4..13u8).rev() {
        let mask = 0b11111 << (high - 4);
        if present & mask == mask {
            return Some(high);
        }
    }
    // Wheel: A-2-3-4-5 (ranks 12,0,1,2,3)
    if present & 0b1_0000_0000_1111 == 0b1_0000_0000_1111 {
        return Some(3); // high card is the 5
    }
    None
}

// Pack ranks sorted by (count desc, rank desc) into 5 nibbles.
fn tiebreak_key(counts: u64) -> u32 {
    // Collect (count, rank) pairs
    let mut pairs: [(u8, u8); 5] = [(0, 0); 5];
    let mut n = 0usize;
    for rank in (0..13u8).rev() {
        let cnt = ((counts >> (rank * 4)) & 0xF) as u8;
        if cnt > 0 {
            pairs[n] = (cnt, rank);
            n += 1;
        }
    }
    // Sort by count desc (rank already desc from the loop above)
    pairs[..n].sort_unstable_by(|a, b| b.0.cmp(&a.0));

    // Pack: first entry = highest bits
    let mut key = 0u32;
    for (i, &(_, rank)) in pairs[..n].iter().enumerate() {
        key |= (rank as u32) << ((4 - i) * 4);
    }
    key
}

fn max_count(counts: u64) -> u8 {
    (0..13).map(|r| ((counts >> (r * 4)) & 0xF) as u8).max().unwrap()
}

fn has_count(counts: u64, target: u8) -> bool {
    (0..13).any(|r| ((counts >> (r * 4)) & 0xF) as u8 == target)
}

fn two_pairs(counts: u64) -> bool {
    (0..13).filter(|&r| ((counts >> (r * 4)) & 0xF) == 2).count() == 2
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
