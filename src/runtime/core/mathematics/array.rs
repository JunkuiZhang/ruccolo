#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Array<T>(pub [T; 4]);

impl<T> Default for Array<T>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Array([T::default(); 4])
    }
}

impl<T> Array<T> {
    pub fn new(data: [T; 4]) -> Self {
        Array(data)
    }
}

impl<T> std::ops::Add<Array<T>> for Array<T>
where
    T: std::ops::Add<T, Output = T> + Default + Copy,
{
    type Output = Array<T>;

    fn add(self, rhs: Array<T>) -> Self::Output {
        let mut res = [T::default(); 4];
        for ((lhs, rhs), sum) in self.0.iter().zip(&rhs.0).zip(&mut res) {
            *sum = *lhs + *rhs;
        }

        return Array(res);
    }
}

impl<T> std::ops::Sub<Array<T>> for Array<T>
where
    T: std::ops::Sub<T, Output = T> + Default + Copy,
{
    type Output = Array<T>;

    fn sub(self, rhs: Array<T>) -> Self::Output {
        let mut res = [T::default(); 4];
        for ((lhs, rhs), sum) in self.0.iter().zip(&rhs.0).zip(&mut res) {
            *sum = *lhs - *rhs;
        }

        return Array(res);
    }
}

impl<T> std::ops::Mul<T> for Array<T>
where
    T: std::ops::Mul<T, Output = T> + Copy,
{
    type Output = Array<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        for index in 0..4 {
            self.0[index] = self.0[index] * rhs;
        }
        self
    }
}

impl<T> PartialEq for Array<T>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

mod test {
    use super::Array;

    #[test]
    fn array_add() {
        let a = Array::new([1; 4]);
        let b = Array::new([2; 4]);
        assert_eq!(a + b, Array::new([3; 4]));
    }

    #[test]
    fn array_sub() {
        let a = Array::new([1; 4]);
        let b = Array::new([2; 4]);
        assert_eq!(b - a, Array::new([1; 4]));
    }

    #[test]
    fn array_mul() {
        let a = Array::new([1; 4]);
        let b = 3;
        assert_eq!(a * b, Array::new([3; 4]));
    }

    // #[test]
    // fn array_div() {
    //     let a = Matrix::new([1; 4]);
    //     let b = Matrix::new([2; 4]);
    //     assert_eq!(b / a, Matrix::new([2; 4]));
    // }

    // #[test]
    // fn array_dot() {
    //     let a = Matrix::new([1; 4]);
    //     let b = Matrix::new([1; 4]);
    //     assert_eq!(a.dot(b), 4);
    // }
}
