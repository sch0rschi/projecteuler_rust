use bitvec::bitvec;
use bitvec::prelude::BitVec;
use itertools::Itertools;
use regex::Regex;
use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let result = solve_0096();
    let duration = start.elapsed();
    println!("{}", result);
    println!("Elapsed: {:?}", duration);
    assert_eq!(24702, result);
    assert!(duration < std::time::Duration::from_secs(1));
}

fn solve_0096() -> u32 {
    let text = fs::read_to_string("resources/0096_sudoku.txt").expect("Failed to read file");

    let re = Regex::new(r"Grid \d+\n").unwrap();

    re.split(&text)
        .skip(1)
        .map(SudokuGrid::new)
        .map(|grid| {
            let solved_grid = grid.solve();
            100 * solved_grid.closed[0][0] as u32
                + 10 * solved_grid.closed[0][1] as u32
                + solved_grid.closed[0][2] as u32
        })
        .sum()
}

#[derive(Clone)]
pub struct SudokuGrid {
    fixed_count: u64,
    closed: [[u8; 9]; 9],
    open: [[BitVec; 9]; 9],
}

impl SudokuGrid {
    pub fn new(input: &str) -> SudokuGrid {
        let lines = input.lines().collect_vec();
        let mut closed = [[0u8; 9]; 9];
        for (line_index, &line) in lines.iter().enumerate() {
            closed[line_index] = line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect_vec()
                .try_into()
                .unwrap();
        }
        let open =
            std::array::from_fn(|_row| std::array::from_fn(|_column| bitvec!(0b111_111_111; 9)));

        let fixed_count = closed.iter().flatten().filter(|&&x| x != 0).count() as u64;

        SudokuGrid {
            fixed_count,
            closed,
            open,
        }
    }

    pub fn print(&self) {
        for (i, row) in self.closed.iter().enumerate() {
            if i % 3 == 0 && i != 0 {
                println!("------+-------+------");
            }

            for (j, cell) in row.iter().enumerate() {
                if j % 3 == 0 && j != 0 {
                    print!("| ");
                }

                if *cell == 0 {
                    print!(". ");
                } else {
                    print!("{} ", cell);
                }
            }

            println!();
        }
    }

    pub fn solve(self) -> SudokuGrid {
        if let Some(sudoku_grid) = self.backtrack() {
            return sudoku_grid;
        }
        panic!();
    }

    pub fn backtrack(mut self) -> Option<SudokuGrid> {
        self.solve_trivial();
        if self.fixed_count == 81 {
            return Some(self);
        }
        let (row_index, column_index) = self.find_undecided();
        for fix_value in self.open[row_index][column_index].iter_ones() {
            let mut new_self = self.clone();
            new_self.place(row_index, column_index, fix_value as u8 + 1);
            if let Some(sudoku_grid) = new_self.backtrack() {
                return Some(sudoku_grid);
            }
        }
        None
    }

    pub fn solve_trivial(&mut self) {
        self.apply_conflicts();
        while self.set_trivial() {
            self.apply_conflicts()
        }
    }

    pub fn apply_conflicts(&mut self) {
        for row in 0..9 {
            for column in 0..9 {
                let cell_value = self.closed[row][column];
                if cell_value != 0 {
                    for index in 0..9 {
                        self.open[row][index].set(cell_value as usize - 1, false);
                        self.open[index][column].set(cell_value as usize - 1, false);
                    }
                    let block_row_start = (row / 3) * 3;
                    let block_column_start = (column / 3) * 3;
                    for block_row in block_row_start..block_row_start + 3 {
                        for block_column in block_column_start..block_column_start + 3 {
                            self.open[block_row][block_column].set(cell_value as usize - 1, false);
                        }
                    }
                }
            }
        }
    }

    pub fn set_trivial(&mut self) -> bool {
        let mut change = false;

        for r in 0..9 {
            for c in 0..9 {
                if self.closed[r][c] != 0 {
                    continue;
                }

                if self.open[r][c].count_ones() == 1 {
                    let v = self.open[r][c].first_one().unwrap() as u8 + 1;
                    self.place(r, c, v);
                    change = true;
                }
            }
        }

        for digit in 0..9 {
            for r in 0..9 {
                let mut cells = [(0usize, 0usize); 9];
                for (c, cell) in cells.iter_mut().enumerate() {
                    *cell = (r, c);
                }

                if let Some((r, c)) = self.scan_unit(&cells, digit) {
                    self.place(r, c, (digit as u8) + 1);
                    change = true;
                }
            }

            for c in 0..9 {
                let mut cells = [(0usize, 0usize); 9];
                for (r, cell) in cells.iter_mut().enumerate() {
                    *cell = (r, c);
                }

                if let Some((r, c)) = self.scan_unit(&cells, digit) {
                    self.place(r, c, (digit as u8) + 1);
                    change = true;
                }
            }

            for br in 0..3 {
                for bc in 0..3 {
                    let mut cells = [(0usize, 0usize); 9];
                    let mut i = 0;

                    for r in 0..3 {
                        for c in 0..3 {
                            cells[i] = (br * 3 + r, bc * 3 + c);
                            i += 1;
                        }
                    }

                    if let Some((r, c)) = self.scan_unit(&cells, digit) {
                        self.place(r, c, (digit as u8) + 1);
                        change = true;
                    }
                }
            }
        }

        change
    }

    fn scan_unit(
        self: &SudokuGrid,
        cells: &[(usize, usize)],
        digit: usize,
    ) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut pos = None;

        for &(r, c) in cells {
            if self.closed[r][c] == 0 && self.open[r][c][digit] {
                count += 1;
                pos = Some((r, c));
                if count > 1 {
                    return None;
                }
            }
        }

        if count == 1 { pos } else { None }
    }

    fn place(self: &mut SudokuGrid, r: usize, c: usize, val: u8) {
        self.closed[r][c] = val;
        self.fixed_count += 1;

        let idx = (val - 1) as usize;

        self.open[r][c].fill(false);

        for i in 0..9 {
            self.open[r][i].set(idx, false);
            self.open[i][c].set(idx, false);
        }

        let br = (r / 3) * 3;
        let bc = (c / 3) * 3;

        for rr in br..br + 3 {
            for cc in bc..bc + 3 {
                self.open[rr][cc].set(idx, false);
            }
        }
    }

    fn find_undecided(&self) -> (usize, usize) {
        for (row_index, row) in self.closed.iter().enumerate() {
            for (column_index, &cell_value) in row.iter().enumerate() {
                if cell_value == 0u8 {
                    return (row_index, column_index);
                }
            }
        }
        panic!()
    }
}
