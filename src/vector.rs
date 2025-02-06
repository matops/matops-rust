use std::array;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T: fmt::Display + fmt::Debug, const N: usize> fmt::Display for Vector<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector").field("data", &self.data).finish()
    }
}

// Vector addition
impl<T: Add<Output = T> + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: array::from_fn(|i| self.data[i] + rhs.data[i]),
        }
    }
}

// Vector subtraction
impl<T: Sub<Output = T> + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: array::from_fn(|i| self.data[i] - rhs.data[i]),
        }
    }
}

// Vector multiplication
impl<T: Mul<Output = T> + Add<Output = T> + Default + Copy, const N: usize> Vector<T, N> {
    pub fn dot(&self, other: &Self) -> T {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .fold(T::default(), |acc, x| acc + x)
    }
}

// Scalar multiplication
impl<T: Mul<Output = T> + Copy, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            data: array::from_fn(|i| self.data[i] * scalar),
        }
    }
}

impl<T: Copy + Into<f64>, const N: usize> Vector<T, N> {
    pub fn magnitude(&self) -> f64 {
        self.data
            .iter()
            .fold(0.0, |acc, &component| {
                let c: f64 = component.into();
                acc + c * c
            })
            .sqrt()
    }
}

impl<T: Copy + Add<Output = T> + Div<Output = T> + From<u8>, const N: usize> Vector<T, N> {
    pub fn halfway(first: &Self, second: &Self) -> Self {
        Self {
            data: array::from_fn(|i| (first.data[i] + second.data[i]) / T::from(2)),
        }
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T> + Sub<Output = T> + From<f64>> Vector<T, 2> {
    pub fn rotate(&self, angle_degrees: f64) -> Self {
        let angle_radians = angle_degrees.to_radians();
        let cos = angle_radians.cos();
        let sin = angle_radians.sin();

        let x = self.data[0];
        let y = self.data[1];

        Self {
            data: [
                T::from(cos) * x - T::from(sin) * y,
                T::from(sin) * x + T::from(cos) * y,
            ],
        }
    }
}
