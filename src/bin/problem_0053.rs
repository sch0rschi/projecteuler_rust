use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0053, 4075);
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
