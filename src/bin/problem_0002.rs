use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0002();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(4613732, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0002() -> i32 {
    let mut f_np = 1;
    let mut f_n = 2;
    let mut sum = f_n;

    while f_n <= 4000000 {
        let temp = f_n;
        f_n += f_np;
        f_np = temp;
        if f_n % 2 == 0 {
            sum += f_n;
        }
    }

    sum
}