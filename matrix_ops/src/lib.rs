use std::ops::{ Add, Sub };
use lalgebra_scalar::Scalar;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);
impl <T: Scalar<Item = T> + std::clone::Clone> Matrix<T> {
	pub fn new() -> Matrix<T> {
        return Matrix(vec![]);
	}
	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        return Matrix(vec![vec![T::zero(); col]; row]);
	}
	pub fn identity(n: usize) -> Matrix<T> {
        let mut result = Matrix::zero(n, n);
        for i in 0..n{
            result.0[i][i] = T::one();
        }
        return result;
	}
}

impl <T> Add for Matrix<T> 
where
    T: Scalar + Scalar<Item = T> + Clone + Add<Output = T> + std::ops::AddAssign,
{
    type Output = Option<Matrix<T>>;
    fn add(self, other: Self) -> Self::Output {
        let self_row_len = self.0.len();
        let self_col_len = self.0[0].len();
        let other_row_len = other.0.len();

        if self_row_len != other_row_len{
            return  None;
        }
        for row_ind in 0..self_row_len{
            if self.0[row_ind].len() != other.0[row_ind].len(){
                return None;
            }
        }
        
        let mut result = Matrix::<T>::zero(self_row_len, self_col_len);
        for row in 0..self_row_len{
            for col in 0..self_col_len{
                result.0[row][col] = self.0[row][col].clone() + other.0[row][col].clone(); 
            }
        }
        return Some(result);

    }
}

impl <T:Scalar<Item = T>  + Clone + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Self) -> Self::Output {
        let self_row_len = self.0.len();
        let self_col_len = self.0[0].len();
        let other_row_len = other.0.len();
        
        if self_row_len != other_row_len{
            return  None;
        }
        for row_ind in 0..self_row_len{
            if self.0[row_ind].len() != other.0[row_ind].len(){
                return None;
            }
        }
        
        let mut result = Matrix::<T>::zero(self_row_len, self_col_len);
        for row in 0..self_row_len{
            for col in 0..self_col_len{
                result.0[row][col] = self.0[row][col].clone() - other.0[row][col].clone(); 
            }
        }
        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}
