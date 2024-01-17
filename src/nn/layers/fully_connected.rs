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
/// let fc: FullyConnected = FullyConnected::new();
/// ```
pub struct FullyConnected;

impl FullyConnected {
    pub fn new() -> Self {
        FullyConnected {

        }
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
        let _layer: FullyConnected = FullyConnected::new();
    }
}
