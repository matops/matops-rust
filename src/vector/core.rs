#[derive(Copy, Clone, PartialEq)]
pub struct Vector<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> Vector<T, N> {
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let v = Vector::new([1, 2, 3]);
        assert_eq!(v.data, [1, 2, 3]);
    }
}
