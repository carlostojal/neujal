use std::sync::mpsc::TrySendError::Full;
use neujal::nn::sequential::Sequential;
use neujal::nn::layers::fully_connected::FullyConnected;

#[test]
fn create_sequential() {
    let model: Sequential = Sequential::new();
    assert_eq!(model.get_layers().len(), 0);
}

#[test]
fn add_layers() {
    let mut model: Sequential = Sequential::new();

    model.add(Box::new(FullyConnected::new()));
    model.add(Box::new(FullyConnected::new()));

    assert_eq!(model.get_layers().len(), 2);
}