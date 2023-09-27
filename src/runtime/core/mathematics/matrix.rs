use super::array::Array;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Matrix<T, const C: usize>([Array<T>; C]);

impl<T, const C: usize> Matrix<T, C>
where
    T: Default + Copy,
{
    pub fn new(data: [[T; 4]; C]) -> Self {
        let mut inner = [Array::<T>::default(); C];
        for (res, src) in inner.iter_mut().zip(&data) {
            *res = Array(*src);
        }
        Matrix(inner)
    }
}

impl<T, const C: usize> Default for Matrix<T, C>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Matrix([Array::<T>::default(); C])
    }
}

impl<T> std::ops::Mul<Array<T>> for Matrix<T, 4>
where
    T: std::ops::Mul<T, Output = T> + std::ops::Add<T, Output = T> + Default + Copy,
{
    type Output = Array<T>;

    fn mul(self, rhs: Array<T>) -> Self::Output {
        return self.0[0] * rhs.0[0]
            + self.0[1] * rhs.0[1]
            + self.0[2] * rhs.0[2]
            + self.0[3] * rhs.0[3];
    }
}

impl<T, const C: usize> std::ops::Add<Matrix<T, C>> for Matrix<T, C>
where
    T: std::ops::Add<T, Output = T> + Default + Copy,
{
    type Output = Matrix<T, C>;

    fn add(self, rhs: Matrix<T, C>) -> Self::Output {
        let mut res = Matrix::<T, C>::default();
        for ((lhs, rhs), num) in self.0.iter().zip(&rhs.0).zip(&mut res.0) {
            *num = *lhs + *rhs;
        }
        return res;
    }
}

impl<T, const C: usize> std::ops::Sub<Matrix<T, C>> for Matrix<T, C>
where
    T: std::ops::Sub<T, Output = T> + Default + Copy,
{
    type Output = Matrix<T, C>;

    fn sub(self, rhs: Matrix<T, C>) -> Self::Output {
        let mut res = Matrix::<T, C>::default();
        for ((lhs, rhs), num) in self.0.iter().zip(&rhs.0).zip(&mut res.0) {
            *num = *lhs - *rhs;
        }
        return res;
    }
}

impl<T> std::ops::Mul<Matrix<T, 4>> for Matrix<T, 4>
where
    T: std::ops::Add<T, Output = T> + std::ops::Mul<T, Output = T> + Default + Copy,
{
    type Output = Matrix<T, 4>;

    fn mul(self, rhs: Matrix<T, 4>) -> Self::Output {
        let mut res = Matrix::<T, 4>::default();
        res.0[0] = self * rhs.0[0];
        res.0[1] = self * rhs.0[1];
        res.0[2] = self * rhs.0[2];
        res.0[3] = self * rhs.0[3];
        return res;
    }
}

impl<T, const C: usize> PartialEq for Matrix<T, C>
where
    T: Eq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[allow(unused_imports)]
mod test {
    use crate::runtime::core::mathematics::array::Array;

    use super::Matrix;

    #[test]
    fn matrix_add() {
        let a = Matrix::new([[1; 4]; 4]);
        let b = Matrix::new([[2; 4]; 4]);
        assert_eq!(a + b, Matrix::new([[3; 4]; 4]));
        let a = Matrix::new([[1; 4]; 2]);
        let b = Matrix::new([[2; 4]; 2]);
        assert_eq!(a + b, Matrix::new([[3; 4]; 2]));
    }

    #[test]
    fn matrix_sub() {
        let a = Matrix::new([[1; 4]; 4]);
        let b = Matrix::new([[2; 4]; 4]);
        assert_eq!(b - a, Matrix::new([[1; 4]; 4]));
        let a = Matrix::new([[1; 4]; 3]);
        let b = Matrix::new([[2; 4]; 3]);
        assert_eq!(b - a, Matrix::new([[1; 4]; 3]));
    }

    #[test]
    fn matrix_mul() {
        let a = Matrix::new([[1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]]);
        let b = Matrix::new([[2; 4]; 4]);
        assert_eq!(b * a, Matrix::new([[2; 4]; 4]));
    }

    #[test]
    fn matrix_mul_array() {
        let a = Matrix::new([[3; 4]; 4]);
        let b = Array::new([1; 4]);
        assert_eq!(a * b, Array::new([12; 4]));
    }
}
