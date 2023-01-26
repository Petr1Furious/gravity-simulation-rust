use std::marker::Copy;
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Default)]
pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Pair<T>
where
    T: Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Pair {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Mul<Output = T>> Mul<T> for Pair<T>
where
    T: Copy,
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Pair {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T, K> From<(K, K)> for Pair<T>
where
    K: Into<T>,
{
    fn from(value: (K, K)) -> Self {
        Pair {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}

impl<T, K> From<[K; 2]> for Pair<T>
where
    K: Into<T> + Copy,
{
    fn from(value: [K; 2]) -> Self {
        Pair {
            x: value[0].into(),
            y: value[1].into(),
        }
    }
}
