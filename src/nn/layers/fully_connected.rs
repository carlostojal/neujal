use crate::nn::layers::layer::Layer;

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
        let layer: FullyConnected = FullyConnected::new();
    }
}