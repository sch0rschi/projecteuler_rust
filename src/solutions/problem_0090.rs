pub fn solve_0090() -> usize {
    let dice: Vec<u16> = (0u16..1024)
        .filter(|m| m.count_ones() == 6)
        .map(|m| if m & (1 << 9) != 0 { m | (1 << 6) } else { m })
        .collect();

    const SQUARES: [(u16, u16); 8] = [
        (1 << 0, 1 << 1),
        (1 << 0, 1 << 4),
        (1 << 0, 1 << 6),
        (1 << 1, 1 << 6),
        (1 << 2, 1 << 5),
        (1 << 3, 1 << 6),
        (1 << 4, 1 << 6),
        (1 << 8, 1 << 1),
    ];

    let mut count = 0;
    for (i, &d1) in dice.iter().enumerate() {
        for &d2 in &dice[i..] {
            if SQUARES
                .iter()
                .all(|&(a, b)| (d1 & a != 0 && d2 & b != 0) || (d1 & b != 0 && d2 & a != 0))
            {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0090::solve_0090;

    #[test]
    fn test() {
        solve_print_and_check(solve_0090, 1217);
    }
}
