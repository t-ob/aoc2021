use std::{
    cmp::max,
    collections::{BinaryHeap, HashSet, VecDeque},
    str::FromStr,
};

use common::io::stdin;

const MAX_HEIGHT: u8 = 9;

#[derive(Debug)]
struct HeightMap {
    heights: [[u8; 1 << 7]; 1 << 7],
    pub rows: usize,
    pub cols: usize,
}

impl HeightMap {
    fn get(&self, idx: (usize, usize)) -> u8 {
        let (row, col) = idx;
        self.heights[row][col]
    }

    fn neighbour_indices(&self, idx: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        match idx {
            (0, 0) => [Some((0, 1)), None, None, Some((1, 0))],
            (0, col) if col == self.cols - 1 => [None, None, Some((0, col - 1)), Some((1, col))],
            (0, col) => [Some((0, col + 1)), None, Some((0, col - 1)), Some((1, col))],
            (row, 0) if row == self.rows - 1 => [Some((row, 1)), Some((row - 1, 0)), None, None],
            (row, col) if row == self.rows - 1 && col == self.cols - 1 => {
                [None, Some((row - 1, col)), Some((row, col - 1)), None]
            }
            (row, col) if row == self.rows - 1 => [
                Some((row, col + 1)),
                Some((row - 1, col)),
                Some((row, col - 1)),
                None,
            ],
            (row, 0) => [Some((row, 1)), Some((row - 1, 0)), None, Some((row + 1, 0))],
            (row, col) if col == self.cols - 1 => [
                None,
                Some((row - 1, col)),
                Some((row, col - 1)),
                Some((row + 1, col)),
            ],
            (row, col) => [
                Some((row, col + 1)),
                Some((row - 1, col)),
                Some((row, col - 1)),
                Some((row + 1, col)),
            ],
        }
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut heights = [[MAX_HEIGHT; 1 << 7]; 1 << 7];

        let mut cols = 0;
        let mut rows = 0;

        for (y, line) in s.lines().enumerate() {
            rows += 1;
            for (x, c) in line.trim().chars().enumerate() {
                cols = max(cols, x + 1);
                if !c.is_numeric() {
                    return Err(());
                }
                heights[y][x] = c as u8 - b'0';
            }
        }

        Ok(HeightMap {
            heights,
            rows,
            cols,
        })
    }
}

fn main() {
    let height_map = match stdin::read::<HeightMap>() {
        Some(height_map) => height_map,
        _ => return,
    };

    let mut min_heights = Vec::new();

    // Part 1
    for row in 0..height_map.rows {
        for col in 0..height_map.cols {
            let idx = (row, col);
            let height = height_map.get(idx);
            let neighbour_indices = height_map.neighbour_indices(idx);

            let min_neighbouring_height = neighbour_indices
                .iter()
                .filter_map(|x| x.map(|idx| height_map.get(idx)))
                .min()
                .unwrap();

            if height < min_neighbouring_height {
                min_heights.push(idx);
            }
        }
    }

    println!("{}", min_heights.iter().map(|idx| 1 + height_map.get(*idx) as u16).sum::<u16>());

    // Part 2
    let mut component_sizes = BinaryHeap::new();

    let mut seen = HashSet::new();

    let mut queue = VecDeque::new();
    while let Some(next_remaining) = min_heights.pop() {
        let mut component_size = 0;
        queue.push_back(next_remaining);
        while let Some(next_idx) = queue.pop_front() {
            if seen.contains(&next_idx) {
                continue;
            }
            seen.insert(next_idx);
            component_size += 1;

            for neighbour_index in height_map.neighbour_indices(next_idx).into_iter().flatten() {
                if seen.contains(&neighbour_index) || height_map.get(neighbour_index) == MAX_HEIGHT {
                    continue;
                }
                queue.push_back(neighbour_index); 
            }

        }
        component_sizes.push(component_size);
    }

    println!("{:?}", component_sizes.iter().take(3).product::<u32>());
}
