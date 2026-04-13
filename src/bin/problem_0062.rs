use rustc_hash::FxHashMap as HashMap;
use std::time::Instant;
use projecteuler::digits::get_digit_count_encoding_15_max;

fn main() {
    let start = Instant::now();
    let result = solve_0062();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(127035954683, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0062() -> u64 {
    let mut cube_lower: u64 = 10;

    let mut digits_map: HashMap<u64, (u8, u64)> = HashMap::default();
    digits_map.reserve(1_000);

    loop {
        digits_map.clear();
        let lower = (cube_lower as f64).cbrt().ceil() as u64;
        let upper = (10f64 * cube_lower as f64).cbrt().floor() as u64;

        for n in lower..=upper {
            let cube = n * n * n;
            let key = get_digit_count_encoding_15_max(cube);

            let entry = digits_map.entry(key).or_insert((0, cube));
            entry.0 += 1;

            if entry.0 == 5 {
                return entry.1;
            }
        }

        cube_lower *= 10;
    }
}
