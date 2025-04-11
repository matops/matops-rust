use super::core::Vector;
use std::array;
use std::ops::{Add, Mul, Sub};

impl<T: Add<Output = T> + Copy, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            data: array::from_fn(|i| self.data[i] + rhs.data[i]),
        }
    }
}

impl<T: Sub<Output = T> + Copy, const N: usize> Sub for Vector<T, N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            data: array::from_fn(|i| self.data[i] - rhs.data[i]),
        }
    }
}

impl<T: Mul<Output = T> + Copy, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;
    fn mul(self, scalar: T) -> Self::Output {
        Self {
            data: array::from_fn(|i| self.data[i] * scalar),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let v1 = Vector::new([1.0, 2.0]);
        let v2 = Vector::new([3.0, 4.0]);
        let result = v1 + v2;
        assert_eq!(result.data, [4.0, 6.0]);
    }

    #[test]
    fn test_dot() {
        let v1 = Vector::new([1.0, 2.0]);
        let v2 = Vector::new([3.0, 4.0]);
        assert_eq!(v1.dot(&v2), 11.0);
    }
}
