use std::{fmt::Debug, ops::{Sub, Add}};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Point<T> where T: Add<Output = T> {
    type Output = Point<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Point<T> where T: Sub<Output = T> {
    type Output = Point<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Point<T> where 
    T: Copy,
    T: From<u8>,
    T: Sub<Output = T>,
    T: Add<Output = T> {
    pub fn neighbors(&self) -> [Point<T>; 4] {
        let one = T::from(1);
        [
            Point { x: self.x, y: self.y - one },
            Point { x: self.x - one, y: self.y },
            Point { x: self.x + one, y: self.y },
            Point { x: self.x, y: self.y + one },
        ]
    }

    pub fn neighbors_with_diagonals(&self) -> [Point<T>; 8] {
        let one = T::from(1);
        [
            Point { x: self.x - one, y: self.y - one },
            Point { x: self.x, y: self.y - one },
            Point { x: self.x + one, y: self.y - one },
            Point { x: self.x - one, y: self.y },
            Point { x: self.x + one, y: self.y },
            Point { x: self.x - one, y: self.y + one },
            Point { x: self.x, y: self.y + one },
            Point { x: self.x + one, y: self.y + one },
        ]
    }
}
