use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

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

impl<T: Mul<Output = T> + Add<Output = T> + Default + Copy, const N: usize> Vector<T, N> {
    pub fn dot(&self, other: &Self) -> T {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .fold(T::default(), |acc, x| acc + x)
    }
}
