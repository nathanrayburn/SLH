mod util;

use std::vec;

use util::Number;

#[derive(Debug,PartialEq,Eq)]
pub struct Shape {
    pub rows: usize,
    pub columns: usize,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Matrix<T: Number>(Vec<Vec<T>>);

#[derive(Debug,PartialEq,Eq)]
pub enum ShapeError {
    MalformedData,
    IncompatibleShapes(Shape, Shape),
}

pub fn shape<T: Number>(m: &Vec<Vec<T>>) -> Option<Shape> {

    if m.is_empty() || m[0].is_empty() {
        return None;
    }

    let size = m[0].len();
    for line in m.iter(){
        if(line.len() != size){
            return None;
        }
    }
    
    Some(Shape {
        rows: m.len(),
        columns: size,
    })
}

pub fn check<T: Number>(m: Vec<Vec<T>>) -> Result<Matrix<T>, ShapeError> {
    if shape(&m) == None{
        return Err(ShapeError::MalformedData)
    }

    Ok(Matrix(m))
}

pub fn empty<T: Number>(shape: Shape) -> Matrix<T> {
    todo!()
}

pub fn identity<T: Number> (n: usize) -> Matrix<T> {
    if  n == 0 {
        panic!("Identity matrix of size 0 is not allowed");
    }
    let mut m: Vec<Vec<T>> = vec![vec![0.into(); n]; n];
    for i in 0..n{
        m[i][i] = 1.into();
    }
    Matrix(m)
}

pub fn matsum<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, ShapeError> {
    check(a.0.clone())?;
    check(b.0.clone())?;

    if shape(&a.0) != shape(&b.0) {
        return Err(ShapeError::IncompatibleShapes(shape(&a.0).unwrap(), shape(&b.0).unwrap()));
    }

    let shape = shape(&a.0).unwrap();
    let n = shape.rows;
    let mut r: Vec<Vec<T>> = vec![vec![0.into(); shape.columns]; n];

    for i in 0..shape.rows {
        for j in 0..shape.columns {
            r[i][j] = a.0[i][j] + b.0[i][j];
        }
    }

    Ok(Matrix(r))
}

pub fn matmul<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, ShapeError> {
    check(a.0.clone())?;
    check(b.0.clone())?;

    if shape(&a.0).unwrap().columns != shape(&b.0).unwrap().rows {
        return Err(ShapeError::IncompatibleShapes(shape(&a.0).unwrap(), shape(&b.0).unwrap()));
    }

    let shape_a = shape(&a.0).unwrap();
    let shape_b = shape(&b.0).unwrap();
    let n = shape_a.rows;
    let m = shape_a.columns;
    let p = shape_b.columns;

    let mut r: Vec<Vec<T>> = vec![vec![0.into(); p]; n];

    for i in 0..n {
        for j in 0..p {
            for k in 0..m {
                r[i][j] += a.0[i][k] * b.0[k][j];
            }
        }
    }

    Ok(Matrix(r))
}

pub fn matsum_inline<T: Number>(a: &mut Matrix<T>, b: &Matrix<T>) -> Result<(), ShapeError> {
    check(a.0.clone())?;
    check(b.0.clone())?;

    if shape(&a.0) != shape(&b.0) {
        return Err(ShapeError::IncompatibleShapes(shape(&a.0).unwrap(), shape(&b.0).unwrap()));
    }

    let shape = shape(&a.0).unwrap();
    let n = shape.rows;

    for i in 0..n {
        for j in 0..shape.columns {
            a.0[i][j] += b.0[i][j];
        }
    }

    Ok(())
}

pub fn bidi_mul<T: Number>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>, ShapeError> {
    check(a.0.clone())?;
    check(b.0.clone())?;

    let shape_a = shape(&a.0).unwrap();
    let shape_b = shape(&b.0).unwrap();

    if shape_a.columns == shape_b.rows {
        // Standard matrix multiplication
        let n = shape_a.rows;
        let m = shape_a.columns;
        let p = shape_b.columns;

        let mut r: Vec<Vec<T>> = vec![vec![0.into(); p]; n];
        for i in 0..n {
            for j in 0..p {
                for k in 0..m {
                    r[i][j] += a.0[i][k] * b.0[k][j];
                }
            }
        }
        Ok(Matrix(r))
    } else if shape_a.rows == shape_b.rows && shape_a.columns == shape_b.columns {
        // Element-wise multiplication
        let mut r = a.0.clone();
        for i in 0..shape_a.rows {
            for j in 0..shape_a.columns {
                r[i][j] *= b.0[i][j];
            }
        }
        Ok(Matrix(r))
    } else {
        Err(ShapeError::IncompatibleShapes(shape_a, shape_b))
    }
}

#[cfg(test)]
mod tests {
    use util::matrix;
    use super::*;

    #[test]
    fn get_shape() {
        assert_eq!(shape(&vec![vec![1,2]]), Some(Shape{ rows: 1, columns: 2}));
        assert_eq!(shape(&vec![vec![1], vec![2]]), Some(Shape{ rows: 2, columns: 1}));
        assert_eq!(shape(&vec![vec![1,2], vec![3]]), None);
    }

    #[test]
    fn irregular_shape_check() {
        assert_eq!(check(vec![vec![1,2], vec![3]]), Err(ShapeError::MalformedData))
    }

    #[test]
    fn zero_sized_matrices() {
        assert_eq!(check::<u8>(vec![]), Err(ShapeError::MalformedData));
        assert_eq!(check::<u8>(vec![vec![], vec![]]), Err(ShapeError::MalformedData));
    }

    #[test]
    #[should_panic]
    fn null_identity() {
        identity::<u8>(0);
    }

    #[test]
    fn identities() {
        assert_eq!(identity(2), matrix![1u8,0; 0,1]);
        assert_eq!(identity(4), matrix![1u8,0,0,0; 0,1,0,0; 0,0,1,0; 0,0,0,1])
    }

    #[test]
    fn simple_add() {
        let one: Matrix<i64> = matrix![1, 2, 3; 0, 10, 20];
        let two = matrix![0, 100, 200; 4, 5, 6];
        let three = matrix![1, 102, 203; 4, 15, 26];

        assert_eq!(matsum(&one, &two), Ok(three));
    }

    #[test]
    fn simple_mul() {
        let one: Matrix<i64> = matrix![2; 3];
        let two = matrix![5, 7];
        let three = matrix![10, 14; 15, 21];

        assert_eq!(matmul(&one, &two), Ok(three));
    }

    #[test]
    fn inline_sum() {
        let mut one: Matrix<i64> = matrix![0,1; 1,0];
        let two = matrix![2,0; 0,4];
        matsum_inline(&mut one, &two).expect("matsum_inline fail");
        assert_eq!(one, matrix![2,1; 1,4]);
    }

    #[test]
    fn bidi_test() {
        let a = matrix![2,3];
        let b = matrix![7;11];
        let c = matrix![2,3,5];
        let d = matrix![0,0;0,0];


        assert_eq!(bidi_mul(&a, &b), Ok(matrix![2*7 + 3*11]));
        assert_eq!(bidi_mul(&b, &a), Ok(matrix![2*7, 3*7; 2*11, 3*11]));
        assert_eq!(bidi_mul(&c, &b), Ok(matrix![14, 21, 35; 22, 33, 55]));
        assert_eq!(bidi_mul(&c, &d), Err(ShapeError::IncompatibleShapes(Shape { rows: 1, columns: 3} , Shape { rows: 2, columns: 2 })))

    }

}
