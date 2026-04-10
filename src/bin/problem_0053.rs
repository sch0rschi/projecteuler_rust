use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0053();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(4075, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0053() -> i32 {
    let mut count = 0;

    let row_0 = vec![1];
    let mut row_above = row_0;

    for row_index in 1..=100 {
        let mut index = 1;
        let mut row = Vec::with_capacity(row_above.len() + 1);
        row.push(1);
        for w in row_above.windows(2) {
            let [left, right] = w else { unreachable!() };

            let mut sum = left + right;
            if sum > 1_000_000 {
                sum = 1_000_000;
                row.push(sum);
                count += row_index - 2 * index + 1;
                break;
            }
            index += 1;
            row.push(sum);
        }
        row.push(1);
        row_above = row;
    }

    count
}
