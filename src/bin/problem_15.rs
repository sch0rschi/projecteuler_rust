const N: usize = 20;

fn main() {
    let mut matrix: [[i64; N+1]; N+1] = [[1; N+1]; N+1];
    for x in 1..=N {
        for y in 1..=N {
            matrix[x][y] = matrix[x - 1][y] + matrix[x][y - 1];
        }
    }

    println!("{}", matrix[N][N]);
}
