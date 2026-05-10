use num_traits::abs;


pub fn solve_0085() -> i32 {
    let i_limit = (1..)
        .find(|&i| get_rectangle_containment(i, i) > 2_000_000)
        .unwrap();

    let mut min_difference = get_rectangle_containment(i_limit, i_limit) - 2_000_000;
    let mut area = i_limit * i_limit;
    let mut down = i_limit - 1;
    let mut up = i_limit;

    while down > 0 {
        let current = get_rectangle_containment(down, up);
        let difference = abs(current - 2_000_000i32);
        if difference < min_difference {
            min_difference = difference;
            area = down * up;
        }
        if current >= 2_000_000 {
            down -= 1;
        } else {
            up += 1;
        }
    }

    area
}

fn get_rectangle_containment(a: i32, b: i32) -> i32 {
    a * (a + 1) * b * (b + 1) / 4
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0085::solve_0085;

    #[test]
    fn test() {
        solve_print_and_check(solve_0085, 2772);
    }
}
