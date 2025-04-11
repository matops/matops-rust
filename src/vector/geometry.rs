use super::core::Vector;
use std::array;
use std::ops::{Add, Div, Mul, Sub};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let v = Vector::new([3.0, 4.0]);
        assert!((v.magnitude() - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_rotate() {
        let v: Vector<f64, 2> = Vector::new([1.0, 0.0]);
        let rotated = v.rotate(90.0);
        assert!((rotated.data[0] - 0.0).abs() < 1e-9);
        assert!((rotated.data[1] - 1.0).abs() < 1e-9);
    }
}
