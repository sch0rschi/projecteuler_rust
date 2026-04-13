use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0061();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(28684, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0061() -> u16 {
    let any_polygonal_numbers: [Vec<u16>; 6] = [
        generate_clamped_numbers(&|n| n * (n + 1) / 2), // triangle numbers
        generate_clamped_numbers(&|n| n * n),           // square numbers
        generate_clamped_numbers(&|n| n * (3 * n - 1) / 2), // pentagonal numbers
        generate_clamped_numbers(&|n| n * (2 * n - 1)), // hexagonal numbers
        generate_clamped_numbers(&|n| n * (5 * n - 3) / 2), // heptagonal numbers
        generate_clamped_numbers(&|n| n * (3 * n - 2)), // octagonal numbers
    ];

    let maps: [[Vec<u16>; 100]; 6] = [
        build_front_map(&any_polygonal_numbers[0]),
        build_front_map(&any_polygonal_numbers[1]),
        build_front_map(&any_polygonal_numbers[2]),
        build_front_map(&any_polygonal_numbers[3]),
        build_front_map(&any_polygonal_numbers[4]),
        build_front_map(&any_polygonal_numbers[5]),
    ];

    let mut used = [false; 6];
    used[5] = true;
    for &number in &any_polygonal_numbers[5] {
        if let Some(sum) = walk_tree(&maps, number / 100, number % 100, used, number) {
            return sum;
        }
    }
    unreachable!()
}

fn generate_clamped_numbers(f: &dyn Fn(u16) -> u16) -> Vec<u16> {
    (1..)
        .map(f)
        .take_while(|&p| p <= 9999)
        .filter(|&p| p >= 1000)
        .collect()
}

fn build_front_map(numbers: &[u16]) -> [Vec<u16>; 100] {
    let mut map: [Vec<u16>; 100] = std::array::from_fn(|_| Vec::new());

    numbers.iter().copied().for_each(|n| {
        map[(n / 100) as usize].push(n);
    });

    map
}

fn walk_tree(
    maps: &[[Vec<u16>; 100]; 6],
    cycle_front: u16,
    back: u16,
    taken: [bool; 6],
    sum: u16,
) -> Option<u16> {
    if taken.iter().all(|&b| b) && cycle_front == back {
        return Some(sum);
    }
    for (number_type, &number_type_taken) in taken.iter().enumerate() {
        if number_type_taken {
            continue;
        }
        for &number in &maps[number_type][back as usize] {
            let mut new_taken = taken;
            new_taken[number_type] = true;
            if let Some(result) =
                walk_tree(maps, cycle_front, number % 100, new_taken, sum + number)
            {
                return Some(result);
            }
        }
    }
    None
}
