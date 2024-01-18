use crate::nn::layers::fully_connected::FullyConnected;
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
/// let fc1: FullyConnected = FullyConnected::new(10, 100);
/// let fc2: FullyConnected = FullyConnected::new(100, 10);
///
/// // add the layers
/// seq.add(Box::new(fc1));
/// seq.add(Box::new(fc2));
/// ```
pub struct Sequential {
    layers: Vec<Box<FullyConnected>>
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

    pub fn add(&mut self, layer: Box<FullyConnected>) {
        if self.layers.len() > 0 {
            // verify compatibility of this layer with the last
            if (&self.layers[self.layers.len()-1]).get_out_features() != layer.get_in_features() {
                panic!("Input features of this layer incompatible with last layer in model!");
            }
        }
        self.layers.push(layer);
    }

    pub fn get_layers(&self) -> &Vec<Box<FullyConnected>> {
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
    fn test_add_layer_compatible() {
        let mut model: Sequential = Sequential::new();

        let fc1: FullyConnected = FullyConnected::new(784, 512);
        let fc2: FullyConnected = FullyConnected::new(512, 100);
        let fc3: FullyConnected = FullyConnected::new(100, 10);

        model.add(Box::new(fc1));
        model.add(Box::new(fc2));
        model.add(Box::new(fc3));

        assert_eq!(model.get_layers().len(), 3);
    }

    #[test]
    #[should_panic]
    fn test_add_layer_incompatible() {
        let mut model: Sequential = Sequential::new();

        let fc1: FullyConnected = FullyConnected::new(784, 512);
        let fc2: FullyConnected = FullyConnected::new(256, 100);
        let fc3: FullyConnected = FullyConnected::new(100, 10);

        model.add(Box::new(fc1));
        model.add(Box::new(fc2));
        model.add(Box::new(fc3));
    }
}
