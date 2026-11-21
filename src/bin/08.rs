use std::fmt::Display;

use bitvec::{array::BitArray, bitarr, order::Lsb0};
use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug)]
pub enum Instruction {
    RotateColumn { col: u8, by: u8 },
    RotateRow { row: u8, by: u8 },
    Rect { cols: u8, rows: u8 },
}

#[derive(Debug, Default)]
pub struct Grid {
    // 6 rows, 50 cols = 300 px;
    row_count: usize,
    col_count: usize,
    pixels: BitArray<[u64; 5]>,
}

impl Grid {
    pub fn new() -> Self {
        Self {
            row_count: 6,
            col_count: 50,
            pixels: bitarr![u64, Lsb0; 0; 300],
        }
    }

    pub fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::RotateColumn { col, by } => {
                let existing = (0..self.row_count).fold(Vec::<bool>::new(), |mut acc, r| {
                    acc.push(self.pixels[r * 50 + *col as usize]);
                    acc
                });
                // apply the rotation
                let mut new = vec![false; self.row_count];
                (0..new.len()).for_each(|c| {
                    // only have to move # cells
                    if existing[c] {
                        let to = (c + (*by as usize)).rem_euclid(self.row_count);
                        // println!("move {c} to {to}");
                        new[to] = true;
                    }
                });
                // overwrite the pixels with the rotated values
                (0..new.len()).for_each(|n| {
                    self.pixels.set(n * 50 + *col as usize, new[n]);
                });
                // dbg!(&instruction, &existing, &new);
                // println!("{self}");
            }
            Instruction::RotateRow { row, by } => {
                let existing = (0..self.col_count).fold(Vec::<bool>::new(), |mut acc, c| {
                    acc.push(self.pixels[*row as usize * 50 + c]);
                    acc
                });
                // apply the rotation
                let mut new = vec![false; self.col_count];
                (0..new.len()).for_each(|r| {
                    // only have to move # cells
                    if existing[r] {
                        let to = (r + (*by as usize)).rem_euclid(self.col_count);
                        // println!("move {r} to {to}");
                        new[to] = true;
                    }
                });
                // overwrite the pixels with the rotated values
                (0..new.len()).for_each(|n| {
                    self.pixels.set(*row as usize * 50 + n, new[n]);
                });
                // dbg!(&instruction, &existing, &new);
                // println!("{self}");
            }
            Instruction::Rect { cols, rows } => {
                for c in 0..*cols {
                    for r in 0..*rows {
                        self.pixels.set((r * 50 + c).into(), true);
                    }
                }
            }
        }
    }

    pub fn illuminated(&self) -> u64 {
        self.pixels.count_ones() as u64
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for idx in 0..300 {
            if self.pixels[idx] {
                f.write_str("#")?;
            } else {
                f.write_str(".")?;
            }
            if (idx + 1) % 50 == 0 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

pub fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .lines()
        .map(|line| match line.split_once(" ") {
            Some(("rect", rest)) => {
                let (cols, rows) = rest.split_once("x").unwrap();
                Instruction::Rect {
                    cols: cols.parse().unwrap(),
                    rows: rows.parse().unwrap(),
                }
            }
            Some(("rotate", rest)) => {
                let (lhs, rhs) = rest.split_once("=").unwrap();
                let (typ, _) = lhs.split_once(" ").unwrap();
                let (n, by) = rhs.split_once(" by ").unwrap();
                let n = n.parse().unwrap();
                let by = by.parse().unwrap();
                match typ {
                    "column" => Instruction::RotateColumn { col: n, by },
                    "row" => Instruction::RotateRow { row: n, by },
                    _ => unreachable!(),
                }
            }
            _ => unimplemented!(),
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = Grid::new();
    let instructions = parse_input(input);
    for i in instructions {
        grid.apply(&i);
    }
    Some(grid.illuminated())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new();
    let instructions = parse_input(input);
    for i in instructions {
        grid.apply(&i);
    }
    println!("{grid}");
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
