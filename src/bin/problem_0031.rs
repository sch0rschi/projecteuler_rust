use projecteuler::evaluation_helper::solve_print_and_check;

fn main() {
    solve_print_and_check(solve_0031, 73682);
}

fn solve_0031() -> i32 {
    let mut counter = 0;
    let remainder = 200;
    for i_200 in 0..=remainder / 200 {
        let remainder = remainder - i_200 * 200;
        for i_100 in 0..=remainder / 100 {
            let remainder = remainder - i_100 * 100;
            for i_50 in 0..=remainder / 50 {
                let remainder = remainder - i_50 * 50;
                for i_20 in 0..=remainder / 20 {
                    let remainder = remainder - i_20 * 20;
                    for i_10 in 0..=remainder / 10 {
                        let remainder = remainder - i_10 * 10;
                        for i_5 in 0..=remainder / 5 {
                            let remainder = remainder - i_5 * 5;
                            counter += 1 + remainder / 2;
                        }
                    }
                }
            }
        }
    }
    counter
}
