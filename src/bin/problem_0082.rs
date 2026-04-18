use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0082();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(260324, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0082() -> u32 {
    let weights = fs::read_to_string("resources/0082_matrix.txt")
        .expect("Failed to read file")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| {
            x.split(",")
                .map(|x| {
                    x.trim()
                        .parse::<u32>()
                        .expect("Failed to parse hex integer")
                })
                .collect_vec()
        })
        .collect_vec();

    let bottom_right = (weights.len() - 1, weights[0].len() - 1);

    let mut closed_list = vec![vec![false; bottom_right.1 + 1]; bottom_right.0 + 1];
    let mut heap = BinaryHeap::new();

    for (row_index, row) in weights.iter().enumerate() {
        heap.push(Reverse((row[0], (row_index, 0))));
    }

    while let Some(Reverse((weight, (row, column)))) = heap.pop() {
        if closed_list[row][column] {
            continue;
        }
        if column == bottom_right.1 {
            return weight;
        }
        closed_list[row][column] = true;
        if row > 0 && !closed_list[row - 1][column] {
            heap.push(Reverse((
                weight + weights[row - 1][column],
                (row - 1, column),
            )));
        }
        if row < bottom_right.0 && !closed_list[row + 1][column] {
            heap.push(Reverse((
                weight + weights[row + 1][column],
                (row + 1, column),
            )));
        }
        if column < bottom_right.1 && !closed_list[row][column + 1] {
            heap.push(Reverse((
                weight + weights[row][column + 1],
                (row, column + 1),
            )));
        }
    }

    panic!("Failed to find target");
}
