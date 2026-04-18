use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0081();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(427337, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0081() -> u32 {
    let weights = fs::read_to_string("resources/0081_matrix.txt")
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

    let mut closed_list = vec![vec![false; weights[0].len()]; weights.len()];
    let mut heap = BinaryHeap::new();
    let target = (weights.len() - 1, weights[0].len() - 1);

    closed_list[0][0] = true;
    heap.push(Reverse((weights[0][1] + weights[0][0], (0, 1))));
    heap.push(Reverse((weights[1][0] + weights[0][0], (1, 0))));

    while let Some(Reverse((weight, (row, column)))) = heap.pop() {
        if closed_list[row][column] {
            continue;
        }
        if (row, column) == target {
            return weight;
        }
        closed_list[row][column] = true;
        if row < target.0 && !closed_list[row + 1][column] {
            heap.push(Reverse((
                weight + weights[row + 1][column],
                (row + 1, column),
            )));
        }
        if column < target.1 && !closed_list[row][column + 1] {
            heap.push(Reverse((
                weight + weights[row][column + 1],
                (row, column + 1),
            )));
        }
    }

    panic!("Failed to find target");
}
