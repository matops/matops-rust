use std::array;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, LowerExp};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Debug, const N: usize> Debug for Vector<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector").field("data", &self.data).finish()
    }
}

impl<T, const N: usize> Display for Vector<T, N>
where
    T: LowerExp + Display + PartialOrd + Copy + Into<f64>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // How many significant digits we want.
        let sig_digits = 3;
        f.write_str("[")?;
        for (i, &value) in self.data.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            let abs_val: f64 = value.into().abs();
            if abs_val == 0.0 {
                // For zero, we simply print 0.000 in fixed format.
                write!(f, "{:.*}", sig_digits, 0.0)?
            } else {
                // Determine how many digits are printed before the decimal point
                // for fixed formatting. For values less than 1, we treat that as 0.
                let digits_before = if abs_val < 1.0 {
                    0
                } else {
                    (abs_val.log10().floor() as i32 + 1) as usize
                };

                // Decide whether to use scientific notation.
                // (Here we use scientific notation if the value is very small,
                // very large, or has more than sig_digits digits before the decimal.)
                if abs_val < 1e-3
                    || abs_val >= 1e4
                    || (abs_val >= 1.0 && digits_before > sig_digits)
                {
                    // Use scientific notation.
                    // Using "{:.2e}" prints one digit before the decimal point and two after,
                    // i.e. three significant digits.
                    write!(f, "{:.2e}", value.into())?
                } else {
                    // Use fixed notation.
                    // We choose the number of digits after the decimal so that the total
                    // number of significant digits is (roughly) sig_digits.
                    let decimal_places = if abs_val < 1.0 {
                        // For numbers less than 1, we print sig_digits digits after the "0."
                        sig_digits
                    } else {
                        // For numbers 1 or larger, the number of digits after the decimal
                        // is sig_digits minus the number of digits before the decimal.
                        sig_digits.saturating_sub(digits_before)
                    };
                    write!(f, "{:.*}", decimal_places, value.into())?
                }
            }
        }
        f.write_str("]")
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
