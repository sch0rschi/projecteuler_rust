use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0028, 669171001);
}

fn solve_0028() -> i32 {
    let mut sum = 1;
    let mut last = 1;
    for i in 1..=500 {
        let new_corner = last + 2 * i;
        last = new_corner + 6 * i;
        sum += 2 * new_corner;
        sum += 2 * last;
    }
    sum
}
