use std::time::Instant;

const N: usize = 20;

fn main() {
    let start = Instant::now();
    let result = solve_0015();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(137846528820, result);
    assert!(duration < std::time::Duration::from_secs(1));
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
