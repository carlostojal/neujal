use crate::nn::layers::layer::Layer;
use crate::nn::model::Model;

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
        let model: Sequential = Sequential::new();
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