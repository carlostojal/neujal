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
    in_features: u32,
    out_features: u32
}

impl FullyConnected {
    pub fn new(in_features: u32, out_features: u32) -> Self {
        FullyConnected {
            in_features,
            out_features
        }
    }

    pub fn get_in_features(&self) -> u32 {
        self.in_features
    }

    pub fn get_out_features(&self) -> u32 {
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
