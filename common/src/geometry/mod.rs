use std::{str::FromStr, ops::{Add, Sub, Mul, Div, RangeTo, AddAssign}, process::Output};

#[derive(Debug, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Scalar<T> {
    pub v: T
}

impl<T> Scalar<T> {
    pub fn new(v: T) -> Scalar<T> {
        Scalar { v }
    }
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.trim().split(",").map(|s| s.parse::<T>());
        
        let (x, y) = match [tokens.next(), tokens.next()] {
            [Some(Ok(x)), Some(Ok(y))] => (x, y),
            _ => return Err(())
        };

        Ok(Point { x, y })
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        
        Point { x, y }
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        
        Point { x, y }   
     }
}

impl<T: Copy + Mul<Output=T>> Mul<Point<T>> for Scalar<T> {
    type Output = Point<T>;

    fn mul(self, rhs: Point<T>) -> Self::Output {
        let x = self.v * rhs.x;
        let y = self.v * rhs.y;
        
        Point { x, y }
    }
}

impl<T: Copy + Div<Output=T>> Div<Scalar<T>> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: Scalar<T>) -> Self::Output {
        let x = self.x / rhs.v;
        let y = self.y / rhs.v;
        
        Point { x, y }    
    }

}
