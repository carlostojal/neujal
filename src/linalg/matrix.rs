use std::ops;
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

    pub fn get_shape(&self) -> (usize,usize) {
        self.shape
    }

    pub fn get_values(&self) -> &Vec<Vec<f32>> {
        &self.values
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
    fn test_create_random() {
        let shape: (usize,usize) = (3,3);
        let matrix: Matrix = Matrix::new(shape, true);

        assert_eq!(matrix.shape, shape);
        assert_eq!(matrix.values.len(), shape.0);
        assert_eq!(matrix.values[0].len(), shape.1);
    }
}

