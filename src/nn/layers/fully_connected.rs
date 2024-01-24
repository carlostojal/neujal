use crate::linalg::tensor::Tensor;
use crate::nn::layers::layer::Layer;

/// Fully connected layer.
///
/// In fully connected layers, activation values are affected
/// by all neurons of the previous layer.
///
/// # Examples
///
/// ```
/// use neujal::nn::layers::fully_connected::FullyConnected;
/// let fc: FullyConnected = FullyConnected::new(784, 10);
/// ```
pub struct FullyConnected {
    weights: Tensor, // TODO: initialize
    // TODO: biases
    in_features: usize,
    out_features: usize
}

impl FullyConnected {
    pub fn new(in_features: usize, out_features: usize) -> Self {
        let w: Tensor = Tensor::new((1, in_features, out_features), true);
        FullyConnected {
            weights: w,
            in_features,
            out_features
        }
    }

    pub fn get_in_features(&self) -> usize {
        self.in_features
    }

    pub fn get_out_features(&self) -> usize {
        self.out_features
    }
}

impl Layer for FullyConnected {

    fn forward(&self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let _layer: FullyConnected = FullyConnected::new(10, 10);
    }
}
