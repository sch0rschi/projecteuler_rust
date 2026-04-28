use itertools::Itertools;
use projecteuler::evaluation_helper::solve_print_and_check;
use std::fs;

const VALID_CHARS: [bool; 256] = make_valid_chars();

fn main() {
    solve_print_and_check(solve_0059, 129448);
}

fn solve_0059() -> u32 {
    let u8_ascii_text = fs::read_to_string("resources/0059_cipher.txt")
        .expect("Failed to read file")
        .split(",")
        .map(|x| x.parse::<u8>().expect("Failed to parse hex integer"))
        .collect_vec();

    let key: [u8; 3] = std::array::from_fn(|i| get_key(&u8_ascii_text, i));

    let sum: u32 = u8_ascii_text
        .iter()
        .zip(key.iter().cycle())
        .map(|(x, y)| (x ^ y) as u32)
        .sum();

    sum
}

fn get_key(text: &[u8], offset: usize) -> u8 {
    for potential_key in b'a'..=b'z' {
        if check_each_third_text_position(text, offset, potential_key) {
            return potential_key;
        }
    }
    unreachable!()
}

fn check_each_third_text_position(content: &[u8], offset: usize, key: u8) -> bool {
    for character_in_text in content.iter().dropping(offset).step_by(3) {
        let xored = character_in_text ^ key;
        if !is_valid_xored_value(xored) {
            return false;
        }
    }
    true
}

fn is_valid_xored_value(xored: u8) -> bool {
    VALID_CHARS[xored as usize]
}

const fn make_valid_chars() -> [bool; 256] {
    let mut arr = [false; 256];
    let mut i = b'a';
    while i <= b'z' {
        arr[i as usize] = true;
        i += 1;
    }
    let mut i = b'A';
    while i <= b'Z' {
        arr[i as usize] = true;
        i += 1;
    }
    let mut i = b'0';
    while i <= b'9' {
        arr[i as usize] = true;
        i += 1;
    }
    let punctuation = b" ,.'\":;()[]+/";
    let mut j = 0;
    while j < punctuation.len() {
        arr[punctuation[j] as usize] = true;
        j += 1;
    }
    arr
}
