fn main() {
    let mut matrix: [[i64; 21]; 21] = [[0; 21]; 21];
    for i in 0..21 {
        matrix[i][0] = 1;
        matrix[0][i] = 1;
    }

    for x in 1..21 {
        for y in 1..21 {
            matrix[x][y] = matrix[x-1][y] + matrix[x][y-1];
        }
    }

    println!("{}", matrix[20][20]);

}