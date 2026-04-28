use projecteuler::evaluation_helper::solve_print_and_check;

const N: usize = 20;

fn main() {
    solve_print_and_check(solve_0015, 137846528820);
}
fn solve_0015() -> i64 {
    let mut matrix: [[i64; N + 1]; N + 1] = [[1; N + 1]; N + 1];
    for x in 1..=N {
        for y in 1..=N {
            matrix[x][y] = matrix[x - 1][y] + matrix[x][y - 1];
        }
    }

    matrix[N][N]
}
