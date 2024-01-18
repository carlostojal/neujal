use rand::Rng;

///
/// Tensor data structure.
///
/// 3D tensor of 32-bit floating point numbers.
///
/// # Examples
/// ```
/// use neujal::linalg::tensor::Tensor;
/// let shape: (usize,usize,usize) = (1,1280,764);
/// let tensor: Tensor = Tensor::new(shape, false);
/// ```
pub struct Tensor {

    shape: (usize, usize, usize),
    values: Vec<Vec<Vec<f32>>>
}

impl Tensor {

    pub fn new(shape: (usize, usize, usize), random: bool) -> Self {

        // initialize random number generator
        let mut rng = rand::thread_rng();

        let mut vx: Vec<Vec<Vec<f32>>> = vec![];

        for _x in 0..shape.0 {

            let mut vy : Vec<Vec<f32>> = vec![];

            // fill the "y" dimension
            for _y in 0..shape.1 {

                if random {
                    let mut vz: Vec<f32> = vec![];

                    // fill the "z" dimension with random values
                    for _y in 0..shape.2 {
                        vz.push(rng.gen_range(0.0..1.0))
                    }
                    vy.push(vz);
                } else {
                    vy.push(vec![0.0;shape.2]);
                }
            }

            vx.push(vy);
        }


        Tensor {
            shape,
            values: vx
        }
    }

    pub fn get_shape(&self) -> (usize,usize,usize) {
        self.shape
    }

    pub fn get_values(&self) -> &Vec<Vec<Vec<f32>>> {
        &self.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let shape: (usize,usize,usize) = (2,3,2);
        let tensor: Tensor = Tensor::new(shape, false);

        assert_eq!(tensor.shape, shape);
        assert_eq!(tensor.values.len(), shape.0); // x dimension
        assert_eq!(tensor.values[0].len(), shape.1); // y dimension
        assert_eq!(tensor.values[0][0].len(), shape.2); // z dimension

    }

    #[test]
    fn test_create_random() {
        let shape: (usize,usize,usize) = (2,3,2);
        let tensor: Tensor = Tensor::new(shape, true);

        assert_eq!(tensor.shape, shape);
        assert_eq!(tensor.values.len(), shape.0); // x dimension
        assert_eq!(tensor.values[0].len(), shape.1); // y dimension
        assert_eq!(tensor.values[0][0].len(), shape.2); // z dimension

    }

    #[test]
    fn test_create_huge() {
        let shape: (usize,usize,usize) = (1,1280,764);
        let tensor: Tensor = Tensor::new(shape, false);

        assert_eq!(tensor.shape, shape);
        assert_eq!(tensor.values.len(), shape.0); // x dimension
        assert_eq!(tensor.values[0].len(), shape.1); // y dimension
        assert_eq!(tensor.values[0][0].len(), shape.2); // z dimension

    }
}