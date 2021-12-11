use std::{
    cmp::max,
    ops::{Add, AddAssign, BitAnd, BitOr, BitXor, BitXorAssign, Div, Mul, RemAssign, Sub},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Grid([[u8; 10]; 10]);

impl Grid {
    pub fn translate(&self, delta: (i8, i8)) -> Self {
        let (dr, dc) = delta;
        let mut grid = [[0u8; 10]; 10];

        let shifted_rows = [[0; 10]]
            .iter()
            .cycle()
            .take(max(0, dr) as usize)
            .chain(self.0.iter())
            .chain([[0; 10]].iter().cycle())
            .skip(max(0, -dr) as usize)
            .take(10);
        for (result_row, shifted_row) in grid.iter_mut().zip(shifted_rows) {
            let shifted_columns = [0]
                .iter()
                .cycle()
                .take(max(0, dc) as usize)
                .chain(shifted_row)
                .chain([0].iter().cycle())
                .skip(max(0, -dc) as usize)
                .take(10);
            for (result_value, shifted_value) in result_row.iter_mut().zip(shifted_columns) {
                *result_value = *shifted_value;
            }
        }

        Grid(grid)
    }

    pub fn sum_values(&self) -> u16 {
        self.0.iter().fold(0, |acc, row| {
            acc + row.iter().fold(0, |acc, v| acc + (*v) as u16)
        })
    }
}

impl AddAssign for Grid {
    fn add_assign(&mut self, rhs: Self) {
        for (lhs_r, rhs_r) in self.0.iter_mut().zip(rhs.0) {
            for (lhs_v, rhs_v) in lhs_r.iter_mut().zip(rhs_r) {
                *lhs_v += rhs_v;
            }
        }
    }
}

impl AddAssign<u8> for Grid {
    fn add_assign(&mut self, rhs: u8) {
        for lhs_r in self.0.iter_mut() {
            for lhs_v in lhs_r.iter_mut() {
                *lhs_v += rhs;
            }
        }
    }
}

impl Add for Grid {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (r, (lhs_r, rhs_r)) in grid.iter_mut().zip(self.0.iter().zip(rhs.0)) {
            for (v, (lhs_v, rhs_v)) in r.iter_mut().zip(lhs_r.iter().zip(rhs_r.iter())) {
                *v = lhs_v + rhs_v;
            }
        }
        Grid(grid)
    }
}

impl RemAssign<u8> for Grid {
    fn rem_assign(&mut self, rhs: u8) {
        for r in self.0.iter_mut() {
            for v in r.iter_mut() {
                *v %= rhs;
            }
        }
    }
}

impl Sub for Grid {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (r, (lhs_r, rhs_r)) in grid.iter_mut().zip(self.0.iter().zip(rhs.0)) {
            for (v, (lhs_v, rhs_v)) in r.iter_mut().zip(lhs_r.iter().zip(rhs_r.iter())) {
                *v = lhs_v - rhs_v;
            }
        }
        Grid(grid)
    }
}

impl BitOr for Grid {
    type Output = Grid;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (r, (lhs_r, rhs_r)) in grid.iter_mut().zip(self.0.iter().zip(rhs.0)) {
            for (v, (lhs_v, rhs_v)) in r.iter_mut().zip(lhs_r.iter().zip(rhs_r.iter())) {
                *v = lhs_v | rhs_v;
            }
        }
        Grid(grid)
    }
}

impl BitAnd for Grid {
    type Output = Grid;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (r, (lhs_r, rhs_r)) in grid.iter_mut().zip(self.0.iter().zip(rhs.0)) {
            for (v, (lhs_v, rhs_v)) in r.iter_mut().zip(lhs_r.iter().zip(rhs_r.iter())) {
                *v = lhs_v & rhs_v;
            }
        }
        Grid(grid)
    }
}

impl BitXorAssign for Grid {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (lhs_r, rhs_r) in self.0.iter_mut().zip(rhs.0) {
            for (lhs_v, rhs_v) in lhs_r.iter_mut().zip(rhs_r) {
                *lhs_v ^= rhs_v
            }
        }
    }
}

impl BitXor<u8> for Grid {
    type Output = Grid;

    fn bitxor(self, rhs: u8) -> Self::Output {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (r, lhs_r) in grid.iter_mut().zip(self.0) {
            for (v, lhs_v) in r.iter_mut().zip(lhs_r) {
                *v = lhs_v ^ rhs;
            }
        }
        Grid(grid)
    }
}

impl Div<u8> for Grid {
    type Output = Grid;

    fn div(self, rhs: u8) -> Self::Output {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (r, lhs_r) in grid.iter_mut().zip(self.0) {
            for (v, lhs_v) in r.iter_mut().zip(lhs_r) {
                *v = lhs_v / rhs;
            }
        }
        Grid(grid)
    }
}

impl Mul<u8> for Grid {
    type Output = Grid;

    fn mul(self, rhs: u8) -> Self::Output {
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (r, lhs_r) in grid.iter_mut().zip(self.0) {
            for (v, lhs_v) in r.iter_mut().zip(lhs_r) {
                *v = lhs_v * rhs;
            }
        }
        Grid(grid)
    }
}

impl PartialEq<u8> for Grid {
    fn eq(&self, other: &u8) -> bool {
        for r in self.0.iter() {
            for v in r {
                if v != other {
                    return false;
                }
            }
        }
        true
    }
}

impl From<u8> for Grid {
    fn from(c: u8) -> Self {
        let grid: [[u8; 10]; 10] = [[c; 10]; 10];
        Grid(grid)
    }
}

impl TryInto<Grid> for Vec<u8> {
    type Error = ();

    fn try_into(self) -> Result<Grid, Self::Error> {
        if self.len() != 100 {
            return Err(());
        }
        let mut grid: [[u8; 10]; 10] = [[0; 10]; 10];
        for (idx, value) in self.iter().enumerate() {
            grid[idx / 10][idx % 10] = *value;
        }

        Ok(Grid(grid))
    }
}