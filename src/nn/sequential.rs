use crate::nn::layers::layer::Layer;

pub struct Sequential {
    layers: Vec<Box<dyn Layer>>
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