use phf::phf_map;
use std::fs;
use std::fs::File;
use std::io::Read;
use projecteuler::evaluation_helper::solve_print_and_check;

pub static CARD_ENCODING: phf::Map<&'static str, u8> = phf_map! {
    "2C" => 1,  "2D" => 2,  "2H" => 3,  "2S" => 4,
    "3C" => 5,  "3D" => 6,  "3H" => 7,  "3S" => 8,
    "4C" => 9,  "4D" => 10, "4H" => 11, "4S" => 12,
    "5C" => 13, "5D" => 14, "5H" => 15, "5S" => 16,
    "6C" => 17, "6D" => 18, "6H" => 19, "6S" => 20,
    "7C" => 21, "7D" => 22, "7H" => 23, "7S" => 24,
    "8C" => 25, "8D" => 26, "8H" => 27, "8S" => 28,
    "9C" => 29, "9D" => 30, "9H" => 31, "9S" => 32,
    "TC" => 33, "TD" => 34, "TH" => 35, "TS" => 36,
    "JC" => 37, "JD" => 38, "JH" => 39, "JS" => 40,
    "QC" => 41, "QD" => 42, "QH" => 43, "QS" => 44,
    "KC" => 45, "KD" => 46, "KH" => 47, "KS" => 48,
    "AC" => 49, "AD" => 50, "AH" => 51, "AS" => 52,
};

fn main() {solve_print_and_check(solve_0054, 376);
}

fn solve_0054() -> i32 {
    let lookup_table = load_table();
    let content = fs::read_to_string("resources/0054_poker.txt").expect("Failed to read file");

    let mut count = 0;
    for line in content.lines() {
        let cards = line.split_whitespace().collect::<Vec<&str>>();
        let hand_1_rank = eval_5(&lookup_table, [
            CARD_ENCODING.get(cards[0]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[1]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[2]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[3]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[4]).unwrap().to_owned(),
        ]);
        let hand_2_rank = eval_5(&lookup_table, [
            CARD_ENCODING.get(cards[5]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[6]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[7]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[8]).unwrap().to_owned(),
            CARD_ENCODING.get(cards[9]).unwrap().to_owned(),
        ]);
        if hand_1_rank > hand_2_rank {
            count += 1;
        }
    }

    count
}

fn load_table() -> Vec<u32> {
    let mut file = File::open("resources/HandRanks.dat").unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    buffer
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
        .collect()
}

fn eval_5(lookup_table: &[u32], cards: [u8; 5]) -> u32 {
    // Start at 53 per TwoPlusTwo evaluator
    let mut p = 53usize;

    p = lookup_table[p + cards[0] as usize] as usize;
    p = lookup_table[p + cards[1] as usize] as usize;
    p = lookup_table[p + cards[2] as usize] as usize;
    p = lookup_table[p + cards[3] as usize] as usize;
    p = lookup_table[p + cards[4] as usize] as usize;

    lookup_table[p]
}
