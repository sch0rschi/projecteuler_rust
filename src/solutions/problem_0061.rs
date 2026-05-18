use smallvec::SmallVec;

pub fn solve_0061() -> u32 {
    let generators: [fn(u32) -> u32; 6] = [
        |n| n * (n + 1) / 2,     // triangle
        |n| n * n,               // square
        |n| n * (3 * n - 1) / 2, // pentagonal
        |n| n * (2 * n - 1),     // hexagonal
        |n| n * (5 * n - 3) / 2, // heptagonal
        |n| n * (3 * n - 2),     // octagonal
    ];

    let maps: [[SmallVec<[u16; 16]>; 100]; 6] = std::array::from_fn(|t| build_map(generators[t]));

    let octa = &maps[5];
    for front in 10u32..100 {
        for &start in octa[front as usize].as_slice() {
            let result = walk(
                &maps,
                front as u8,
                (start % 100) as u8,
                0b111111 & !(1 << 5),
                start as u32,
            );
            if let Some(sum) = result {
                return sum;
            }
        }
    }
    unreachable!()
}

fn build_map(f: fn(u32) -> u32) -> [SmallVec<[u16; 16]>; 100] {
    let mut map: [SmallVec<[u16; 16]>; 100] = std::array::from_fn(|_| SmallVec::new());
    let mut n = 1u32;
    loop {
        let v = f(n);
        if v > 9999 {
            break;
        }
        if v >= 1000 {
            map[(v / 100) as usize].push(v as u16);
        }
        n += 1;
    }
    map
}

fn walk(
    maps: &[[SmallVec<[u16; 16]>; 100]; 6],
    cycle_front: u8,
    back: u8,
    need: u8,
    sum: u32,
) -> Option<u32> {
    if need == 0 && back == cycle_front {
        return Some(sum);
    }
    let mut remaining = need;
    while remaining != 0 {
        let t = remaining.trailing_zeros() as usize;
        remaining &= remaining - 1;

        for &number in maps[t][back as usize].iter() {
            let result = walk(
                maps,
                cycle_front,
                (number % 100) as u8,
                need & !(1 << t),
                sum + number as u32,
            );
            if result.is_some() {
                return result;
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0061::solve_0061;

    #[test]
    fn test() {
        solve_print_and_check(solve_0061, 28684);
    }
}
