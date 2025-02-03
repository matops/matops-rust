use std::fmt::Debug;
use std::fmt::Result;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Debug, const N: usize> Debug for Vector<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.debug_struct("Vector").field("data", &self.data).finish()
    }
}

// Vector addition
impl<T: Add<Output = T> + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (i, &x) in rhs.data.iter().enumerate() {
            data[i] = data[i] + x;
        }
        Self { data }
    }
}

// Vector subtraction
impl<T: Sub<Output = T> + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut data = self.data;
        for (i, &x) in rhs.data.iter().enumerate() {
            data[i] = data[i] - x;
        }
        Self { data }
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
        let mut data = self.data;
        for x in &mut data {
            *x = *x * scalar;
        }
        Self { data }
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
