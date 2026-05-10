pub fn solve_0018() -> u32 {
    #[rustfmt::skip]
    let mut tri = vec![
        75,
        95, 64,
        17, 47, 82,
        18, 35, 87, 10,
        20, 4,  82, 47, 65,
        19, 1,  23, 75, 3,  34,
        88, 2,  77, 73, 7,  63, 67,
        99, 65, 4,  28, 6,  16, 70, 92,
        41, 41, 26, 56, 83, 40, 80, 70, 33,
        41, 48, 72, 33, 47, 32, 37, 16, 94, 29,
        53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14,
        70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57,
        91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48,
        63, 66, 4,  68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31,
        4,  62, 98, 27, 23, 9,  70, 98, 73, 93, 38, 53, 60, 4,  23,
    ];

    let rows = 15;

    for row_index in (0..rows - 1).rev() {
        let start = row_index * (row_index + 1) / 2;
        let next_start = (row_index + 1) * (row_index + 2) / 2;

        for column_index in 0..=row_index {
            let i = start + column_index;
            let left = next_start + column_index;
            let right = next_start + column_index + 1;

            tri[i] += tri[left].max(tri[right]);
        }
    }

    tri[0]
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0018::solve_0018;

    #[test]
    fn test() {
        solve_print_and_check(solve_0018, 1074);
    }
}
