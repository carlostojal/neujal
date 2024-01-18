use std::ops;
use std::fmt;
use std::fmt::{Formatter};
use std::ops::{Index, IndexMut};
use rand::Rng;

///
/// Matrix data structure. 
///
/// Matrix data structure. Supports random initialization and multiplication.
///
/// # Examples
/// ```
/// use neujal::linalg::matrix::Matrix;
/// // initialize a 3x3 matrix with zeros
/// let mat: Matrix = Matrix::new((3, 3), false);
/// ```
pub struct Matrix {
    shape: (usize,usize),
    values: Vec<Vec<f32>>
}

impl Matrix {

    pub fn new(shape: (usize,usize), random: bool) -> Self {

        // initialize random number generator
        let mut rng = rand::thread_rng();

        let mut vx: Vec<Vec<f32>> = vec![];

        for _x in 0..shape.0 {

            let mut vy: Vec<f32> = vec![];

            // zero initialize
            if !random {
                vy = vec![0.0;shape.1];
            } else {
                for _y in 0..shape.1 {
                    vy.push(rng.gen_range(0.0..1.0));
                }
            }

            vx.push(vy);
        }

        // return the matrix struct
        Matrix {
            shape,
            values: vx
        }
    }

    pub fn shape(&self) -> (usize,usize) {
        self.shape
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out: String = String::new();

        // for each row
        for _x in 0..self.shape.0 {

            // for each column
            for _y in 0..self.shape.1 {
                out += &format!("{:.2}", self.values[_x][_y]);

                out += "\t";
            }

            // add a new line after each row
            out += "\n";
        }

        write!(f, "{}", out)
    }
}

// overload multiplication operator
impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        // verify shape compatibility
        if self.shape.1 != rhs.shape.0 {
            panic!("Columns of first matrix don't match rows of the second!");
        }

        // declare the output matrix
        let mut out: Matrix = Matrix::new((self.shape.0, rhs.shape.1), false);

        // for each row of the first
        for _x in 0..self.shape.0 {

            // for each column of the second
            for _y in 0..rhs.shape.1 {

                let mut dot_product: f32 = 0.0;

                // for each column of the first / row of the second
                for _z in 0..self.shape.1 {
                    dot_product += self.values[_x][_z] * rhs.values[_z][_y];
                }

                out.values[_x][_y] = dot_product;
            }
        }

        out
    }
}

// overload addition operator
impl ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        // verify shape compatibility
        if self.shape != rhs.shape {
            panic!("First and second matrices shapes should be the same!");
        }

        let mut out: Matrix = Matrix::new(self.shape, false);

        // for each row
        for _x in 0..self.shape.0 {

            // for each column
            for _y in 0..self.shape.1 {

                out.values[_x][_y] = self.values[_x][_y] + rhs.values[_x][_y];
            }
        }

        out
    }
}

// overload index operator
impl Index<usize> for Matrix {
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

// overload mutable index operator
impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let shape: (usize,usize) = (3,3);
        let matrix: Matrix = Matrix::new(shape, false);

        assert_eq!(matrix.shape, shape);
        assert_eq!(matrix.values.len(), shape.0);
        assert_eq!(matrix.values[0].len(), shape.1);
    }

    #[test]
    fn test_create_index_operator() {
        let shape: (usize,usize) = (3,3);
        let matrix: Matrix = Matrix::new(shape, false);

        assert_eq!(matrix.shape, shape);
        assert_eq!(matrix.len(), shape.0);
        assert_eq!(matrix[0].len(), shape.1);
    }

    #[test]
    fn test_create_random() {
        let shape: (usize,usize) = (3,3);
        let matrix: Matrix = Matrix::new(shape, true);

        assert_eq!(matrix.shape, shape);
        assert_eq!(matrix.values.len(), shape.0);
        assert_eq!(matrix.values[0].len(), shape.1);
    }

    #[test]
    fn test_display() {
        let mut matrix: Matrix = Matrix::new((2, 2), false);
        matrix[0][0] = 1.0;
        matrix[0][1] = 2.0;
        matrix[1][0] = 3.0;
        matrix[1][1] = 4.0;

        println!("{}", matrix);

        let s: String = "1.00\t2.00\t\n3.00\t4.00\t\n".to_string();

        assert_eq!(matrix.to_string(), s);
    }

    // TODO: multiplication tests

    // TODO: addition tests
}

