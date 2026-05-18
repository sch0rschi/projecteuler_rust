const INPUT: &str = include_str!("../../resources/0067_triangle.txt");

pub fn solve_0067() -> u64 {
    let mut flat: Vec<u64> = INPUT
        .lines()
        .flat_map(|line| line.split(' ').map(|s| s.parse::<u64>().unwrap()))
        .collect();

    let rows = INPUT.lines().count();
    for row in (0..rows - 1).rev() {
        let curr = row * (row + 1) / 2;
        let next = (row + 1) * (row + 2) / 2;
        for i in 0..=row {
            flat[curr + i] += flat[next + i].max(flat[next + i + 1]);
        }
    }

    flat[0]
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0067::solve_0067;

    #[test]
    fn test() {
        solve_print_and_check(solve_0067, 7273);
    }
}
