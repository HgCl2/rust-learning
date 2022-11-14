#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Matrix((i32,i32), (i32,i32));

pub fn transpose(m: Matrix) -> Matrix {
    let (a, b) = m.0;
    let (c, d) = m.1;
    return Matrix((a, c), (b, d));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix((1, 3), (4, 5));
        let result = transpose(matrix);
        let right = Matrix((1, 4), (3, 5));
        assert_eq!(result, right);
    }
}
