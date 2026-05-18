const INPUT: &str = include_str!("../../resources/0096_sudoku.txt");

type Mask = u16;

const ALL: Mask = 0x1FF;

#[inline(always)]
const fn bit(d: u8) -> Mask {
    1 << (d - 1)
}

#[inline(always)]
const fn singleton_to_digit(mask: Mask) -> u8 {
    mask.trailing_zeros() as u8 + 1
}

const fn build_peers() -> [[usize; 20]; 81] {
    let mut peers = [[0usize; 20]; 81];

    let mut cell = 0;

    while cell < 81 {
        let r = cell / 9;
        let c = cell % 9;

        let mut used = [false; 81];
        used[cell] = true;

        let mut n = 0;

        let mut i = 0;
        while i < 9 {
            let x = r * 9 + i;
            if !used[x] {
                used[x] = true;
                peers[cell][n] = x;
                n += 1;
            }

            let x = i * 9 + c;
            if !used[x] {
                used[x] = true;
                peers[cell][n] = x;
                n += 1;
            }

            i += 1;
        }

        let br = (r / 3) * 3;
        let bc = (c / 3) * 3;

        let mut rr = 0;
        while rr < 3 {
            let mut cc = 0;
            while cc < 3 {
                let x = (br + rr) * 9 + (bc + cc);

                if !used[x] {
                    used[x] = true;
                    peers[cell][n] = x;
                    n += 1;
                }

                cc += 1;
            }

            rr += 1;
        }

        cell += 1;
    }

    peers
}

const PEERS: [[usize; 20]; 81] = build_peers();

#[derive(Clone)]
struct Sudoku {
    grid: [u8; 81],
    cand: [Mask; 81],
}

impl Sudoku {
    fn new(block: &str) -> Self {
        let mut s = Sudoku {
            grid: [0; 81],
            cand: [ALL; 81],
        };

        let mut idx = 0;

        for line in block.lines() {
            for &b in line.as_bytes() {
                let d = b - b'0';

                if d != 0 && !s.assign(idx, d) {
                    panic!("invalid sudoku");
                }

                idx += 1;
            }
        }

        s
    }

    #[inline(always)]
    fn assign(&mut self, idx: usize, digit: u8) -> bool {
        let mask = bit(digit);

        self.grid[idx] = digit;
        self.cand[idx] = mask;

        let mut stack = [0usize; 81];
        let mut top = 0;

        stack[top] = idx;
        top += 1;

        while top > 0 {
            top -= 1;

            let cell = stack[top];

            let value = self.cand[cell];

            for &peer in &PEERS[cell] {
                let old = self.cand[peer];

                if old & value == 0 {
                    continue;
                }

                let new = old & !value;

                if new == 0 {
                    return false;
                }

                if new != old {
                    self.cand[peer] = new;

                    if new.count_ones() == 1 && self.grid[peer] == 0 {
                        self.grid[peer] = singleton_to_digit(new);

                        stack[top] = peer;
                        top += 1;
                    }
                }
            }
        }

        true
    }

    #[inline(always)]
    fn find_best(&self) -> Option<usize> {
        let mut best = None;
        let mut best_count = 10;

        let mut i = 0;

        while i < 81 {
            if self.grid[i] == 0 {
                let c = self.cand[i].count_ones();

                if c < best_count {
                    best_count = c;
                    best = Some(i);

                    if c == 2 {
                        break;
                    }
                }
            }

            i += 1;
        }

        best
    }

    fn solve(&mut self) -> bool {
        let Some(idx) = self.find_best() else {
            return true;
        };

        let mask = self.cand[idx];

        let snapshot = self.clone();

        let mut bits = mask;

        while bits != 0 {
            let lsb = bits & (!bits + 1);

            let digit = singleton_to_digit(lsb);

            let mut next = snapshot.clone();

            if next.assign(idx, digit) && next.solve() {
                *self = next;
                return true;
            }

            bits ^= lsb;
        }

        false
    }
}

pub fn solve_0096() -> u32 {
    let mut total = 0;

    let mut lines = INPUT.lines();

    while lines.next().is_some() {
        let mut block = String::with_capacity(90);

        for _ in 0..9 {
            block.push_str(lines.next().unwrap());
            block.push('\n');
        }

        let mut sudoku = Sudoku::new(&block);

        assert!(sudoku.solve());

        total += 100 * sudoku.grid[0] as u32 + 10 * sudoku.grid[1] as u32 + sudoku.grid[2] as u32;
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::libs::evaluation_helper::solve_print_and_check;
    use crate::problem_0096::solve_0096;

    #[test]
    fn test() {
        solve_print_and_check(solve_0096, 24702);
    }
}
