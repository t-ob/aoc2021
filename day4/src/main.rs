use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use common::io::stdin::{collect_lines_into_vec, read_line_with};

const BOARD_SIZE: usize = 5;

#[derive(Clone)]
struct Board {
    row_mark_count: [usize; BOARD_SIZE],
    col_mark_count: [usize; BOARD_SIZE],
    total: u16,
    marked_total: u16,
    values: HashMap<u16, (usize, usize)>,
}

impl Board {
    pub fn mark(&mut self, value: u16) -> Option<u32> {
        if let Some((row, col)) = self.values.get(&value) {
            self.marked_total += value;
            self.row_mark_count[*row] += 1;
            self.col_mark_count[*col] += 1;

            if self.row_mark_count[*row] == BOARD_SIZE || self.col_mark_count[*col] == BOARD_SIZE {
                return Some(value as u32 * (self.total - self.marked_total) as u32);
            }
        }
        None
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row_mark_count = [0; BOARD_SIZE];
        let col_mark_count = [0; BOARD_SIZE];
        let mut total = 0;
        let marked_total = 0;
        let mut values = HashMap::new();
        for (row, line) in s.lines().enumerate() {
            for (col, s) in line.split_ascii_whitespace().enumerate() {
                if let Ok(value) = s.parse::<u16>() {
                    total += value;
                    values.insert(value, (row, col));
                } else {
                    return Err(());
                }
            }
        }

        Ok(Board {
            row_mark_count,
            col_mark_count,
            total,
            marked_total,
            values,
        })
    }
}

fn main() {
    if let Some(numbers) = read_line_with(|line| {
        line.trim()
            .split(',')
            .map(|s| s.parse::<u16>().ok())
            .collect::<Option<Vec<_>>>()
    }) {
        let boards = collect_lines_into_vec::<Board>("\n\n");

        // Part 1
        let mut boards_part_1 = boards.clone();
        let mut first_found = false;
        for number in &numbers {
            if first_found {
                break;
            }
            for board in boards_part_1.iter_mut() {
                if let Some(score) = board.mark(*number) {
                    println!("{}", score);
                    first_found = true;
                    break;
                }
            }
        }

        // Part 2
        let mut boards_part_2 = boards;
        let mut winning_boards = HashSet::new();
        let mut last_winning_score = None;
        for number in &numbers {
            for (idx, board) in boards_part_2.iter_mut().enumerate() {
                if winning_boards.contains(&idx) {
                    continue;
                }
                if let Some(score) = board.mark(*number) {
                    winning_boards.insert(idx);
                    last_winning_score = Some(score);
                }
            }
        }

        println!("{}", last_winning_score.unwrap());
    }
}
