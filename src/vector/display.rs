use super::core::Vector;
use core::fmt;
use core::fmt::{Debug, Display, Formatter, LowerExp};

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
        let sig_digits = 3;
        f.write_str("[")?;
        for (i, &value) in self.data.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            let abs_val: f64 = value.into().abs();
            if abs_val == 0.0 {
                write!(f, "{:.*}", sig_digits, 0.0)?;
            } else {
                let digits_before = if abs_val < 1.0 {
                    0
                } else {
                    (abs_val.log10().floor() as i32 + 1) as usize
                };
                if abs_val < 1e-3
                    || abs_val >= 1e4
                    || (abs_val >= 1.0 && digits_before > sig_digits)
                {
                    write!(f, "{:.2e}", value.into())?;
                } else {
                    let decimal_places = if abs_val < 1.0 {
                        sig_digits
                    } else {
                        sig_digits.saturating_sub(digits_before)
                    };
                    write!(f, "{:.*}", decimal_places, value.into())?;
                }
            }
        }
        f.write_str("]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug() {
        let v = Vector::new([1, 2]);
        assert_eq!(format!("{:?}", v), "Vector { data: [1, 2] }");
    }

    #[test]
    fn test_display() {
        let v = Vector::new([1.0, 0.0, 1e-5]);
        assert_eq!(format!("{}", v), "[1.00, 0.000, 1.00e-5]");
    }
}
