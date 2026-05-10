use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs;


pub fn solve_0083() -> u32 {
    let weights: Vec<Vec<u32>> = fs::read_to_string("resources/0083_matrix.txt")
        .expect("Failed to read file")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|x| x.trim().parse::<u32>().expect("Failed to parse integer"))
                .collect()
        })
        .collect();

    let rows = weights.len();
    let cols = weights[0].len();
    let target = (rows - 1, cols - 1);

    let mut dist = vec![vec![u32::MAX; cols]; rows];
    let mut heap = BinaryHeap::new();

    dist[0][0] = weights[0][0];
    heap.push(Reverse((weights[0][0], (0usize, 0usize))));

    let directions = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];

    while let Some(Reverse((cost, (row, column)))) = heap.pop() {
        if cost > dist[row][column] {
            continue;
        }

        if (row, column) == target {
            return cost;
        }

        for (delta_row, delta_column) in directions {
            let new_row = row as isize + delta_row;
            let new_column = column as isize + delta_column;

            if new_row < 0
                || new_column < 0
                || new_row >= rows as isize
                || new_column >= cols as isize
            {
                continue;
            }

            let new_row = new_row as usize;
            let new_column = new_column as usize;

            let new_cost = cost + weights[new_row][new_column];
            let cost_ref: &mut u32 = &mut dist[new_row][new_column];

            if new_cost < *cost_ref {
                *cost_ref = new_cost;
                heap.push(Reverse((new_cost, (new_row, new_column))));
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
