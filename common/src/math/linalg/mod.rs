use std::{
    ops::{Add, AddAssign, Div, Mul, Sub},
    str::FromStr, iter::Sum,
};

#[derive(Debug, Clone, Copy)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Scalar<T> {
    pub v: T,
}

impl<T> Scalar<T> {
    pub fn new(v: T) -> Scalar<T> {
        Scalar { v }
    }
}

impl<T: FromStr> FromStr for Vec2D<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split(',').map(|s| s.parse::<T>());

        let (x, y) = match [tokens.next(), tokens.next()] {
            [Some(Ok(x)), Some(Ok(y))] => (x, y),
            _ => return Err(()),
        };

        Ok(Vec2D { x, y })
    }
}

impl<T: Add<Output = T>> Add for Vec2D<T> {
    type Output = Vec2D<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;

        Vec2D { x, y }
    }
}

impl<T: AddAssign> AddAssign for Vec2D<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Vec2D<T> {
    type Output = Vec2D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;

        Vec2D { x, y }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vec2D<T>> for Scalar<T> {
    type Output = Vec2D<T>;

    fn mul(self, rhs: Vec2D<T>) -> Self::Output {
        let x = self.v * rhs.x;
        let y = self.v * rhs.y;

        Vec2D { x, y }
    }
}

impl<T: Copy + Div<Output = T>> Div<Scalar<T>> for Vec2D<T> {
    type Output = Vec2D<T>;

    fn div(self, rhs: Scalar<T>) -> Self::Output {
        let x = self.x / rhs.v;
        let y = self.y / rhs.v;

        Vec2D { x, y }
    }
}

pub fn scalar_product<T: Copy + Sum + Mul<Output = T>>(lhs: &[T], rhs: &[T]) -> T {
    lhs.iter()
        .zip(rhs.iter())
        .map(|(s, t)| *s * *t)
        .sum::<T>()
}