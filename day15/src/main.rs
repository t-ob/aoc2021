use std::{
    cmp::{min, Reverse},
    collections::{BinaryHeap, HashSet},
};

use common::io::stdin;

type Grid = Vec<Vec<usize>>;

const PART_2_MAP: [[usize; 10]; 9] = [
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
    [0, 2, 3, 4, 5, 6, 7, 8, 9, 1],
    [0, 3, 4, 5, 6, 7, 8, 9, 1, 2],
    [0, 4, 5, 6, 7, 8, 9, 1, 2, 3],
    [0, 5, 6, 7, 8, 9, 1, 2, 3, 4],
    [0, 6, 7, 8, 9, 1, 2, 3, 4, 5],
    [0, 7, 8, 9, 1, 2, 3, 4, 5, 6],
    [0, 8, 9, 1, 2, 3, 4, 5, 6, 7],
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
];

fn dijktstra(grid: &Grid) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut distances = vec![vec![usize::MAX; cols]; rows];
    distances[0][0] = 0;

    let mut heap = BinaryHeap::new();

    heap.push((Reverse(0), (0, 0)));

    let mut curr = (0, 0);
    let dest = (rows - 1, cols - 1);

    let mut known = HashSet::new();

    while curr != dest {
        let (_, (row, col)) = match heap.pop() {
            Some(result) => result,
            _ => return 0,
        };

        if known.contains(&(row, col)) {
            continue;
        }

        let deltas = [
            (col < cols - 1, (0, 1)),
            (row > 0, (-1, 0)),
            (col > 0, (0, -1)),
            (row < rows - 1, (1, 0)),
        ];

        for (n_row, n_col) in deltas
            .iter()
            .filter(|(p, _)| *p)
            .map(|(_, (dr, dc))| ((row as i32 + *dr) as usize, (col as i32 + *dc) as usize))
        {
            let n_distance = min(
                distances[n_row][n_col],
                distances[row][col] + grid[n_row][n_col],
            );
            distances[n_row][n_col] = n_distance;
            heap.push((Reverse(n_distance), (n_row, n_col)));
        }

        curr = (row, col);
        known.insert((row, col));
    }

    distances[rows - 1][cols - 1]
}

fn main() {
    let grid = stdin::collect_into_vec_with::<Vec<usize>>(|line| {
        let mut row = vec![];
        for c in line.trim().chars() {
            row.push((c as u8 - '0' as u8) as usize);
        }
        Some(row)
    });

    let mut grid_2 = vec![vec![0; 5 * grid[0].len()]; 5 * grid.len()];
    for (y, row) in grid_2.iter_mut().enumerate() {
        for (x, v) in row.iter_mut().enumerate() {
            *v = PART_2_MAP[x / grid.len() + y / grid[0].len()]
                [grid[y % grid.len()][x % grid[0].len()]];
        }
    }

    println!("{:?}", dijktstra(&grid));
    println!("{:?}", dijktstra(&grid_2));
}
