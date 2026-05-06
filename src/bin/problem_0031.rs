use projecteuler::evaluation_helper::solve_print_and_check;

const COINS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
const LIMIT: usize = 200;

fn main() {
    solve_print_and_check(solve_0031, 73682);
}

fn solve_0031() -> u32 {
    let mut possibilities = [0u32; LIMIT + 1];
    possibilities[0] = 1;

    for coin in COINS {
        for i in coin..LIMIT + 1 {
            possibilities[i] += possibilities[i - coin];
        }
    }

    possibilities[LIMIT]
}
