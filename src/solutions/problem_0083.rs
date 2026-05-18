use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../../resources/0083_matrix.txt");

pub fn solve_0083() -> u32 {
    let mut cols = 0usize;
    let weights: Vec<u32> = INPUT
        .lines()
        .filter(|l| !l.is_empty())
        .flat_map(|line| {
            let row: Vec<u32> = line
                .split(',')
                .map(|x| x.trim().parse::<u32>().expect("Failed to parse integer"))
                .collect();
            cols = row.len();
            row
        })
        .collect();

    let rows = weights.len() / cols;
    let target = rows * cols - 1;

    let mut dist = vec![u32::MAX; rows * cols];
    let mut heap = BinaryHeap::new();

    dist[0] = weights[0];
    heap.push(Reverse((weights[0], 0usize)));

    while let Some(Reverse((cost, idx))) = heap.pop() {
        if cost > dist[idx] {
            continue;
        }
        if idx == target {
            return cost;
        }

        let row = idx / cols;
        let col = idx % cols;

        let neighbours = [
            (row > 0).then(|| idx - cols),
            (row + 1 < rows).then(|| idx + cols),
            (col > 0).then(|| idx - 1),
            (col + 1 < cols).then(|| idx + 1),
        ];

        for next_idx in neighbours.into_iter().flatten() {
            let new_cost = cost + weights[next_idx];
            if new_cost < dist[next_idx] {
                dist[next_idx] = new_cost;
                heap.push(Reverse((new_cost, next_idx)));
            }
        }
    }

    panic!("Failed to find target");
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0083::solve_0083;

    #[test]
    fn test() {
        solve_print_and_check(solve_0083, 425185);
    }
}
