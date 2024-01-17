use crate::nn::layers::layer::Layer;
use crate::nn::model::Model;

///
/// Sequential model.
///
/// The sequential model is a simple to use model.
/// Just add layers to the model, and the execution flow will be automatically sequential.
///
/// # Examples
/// ```
/// use neujal::nn::layers::fully_connected::FullyConnected;
/// use neujal::nn::sequential::Sequential;
/// let mut seq: Sequential = Sequential::new();
///
/// // create the layers
/// let fc1: FullyConnected = FullyConnected::new();
/// let fc2: FullyConnected = FullyConnected::new();
///
/// // add the layers
/// seq.add(Box::new(fc1));
/// seq.add(Box::new(fc2));
/// ```
pub struct Sequential {
    layers: Vec<Box<dyn Layer>>
}

impl Model for Sequential {
    fn forward() {
        todo!()
    }
}

impl Sequential {

    pub fn new() -> Self {
        Sequential {
            layers: Vec::new()
        }
    }

    pub fn add(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    pub fn get_layers(&self) -> &Vec<Box<dyn Layer>> {
        &self.layers
    }
}

#[cfg(test)]
mod tests {
    use crate::nn::layers::fully_connected::FullyConnected;
    use super::*;

    #[test]
    fn test_init() {
        let _model: Sequential = Sequential::new();
    }

    #[test]
    fn test_add_layer() {
        let mut model: Sequential = Sequential::new();

        model.add(Box::new(FullyConnected::new()));
        model.add(Box::new(FullyConnected::new()));
        model.add(Box::new(FullyConnected::new()));

        assert_eq!(model.get_layers().len(), 3);
    }
}
