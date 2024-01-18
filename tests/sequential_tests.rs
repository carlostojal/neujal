use neujal::nn::sequential::Sequential;
use neujal::nn::layers::fully_connected::FullyConnected;

#[test]
fn create_sequential() {
    let model: Sequential = Sequential::new();
    assert_eq!(model.get_layers().len(), 0);
}

#[test]
fn add_layers_compatible() {
    let mut model: Sequential = Sequential::new();

    model.add(Box::new(FullyConnected::new(10, 100)));
    model.add(Box::new(FullyConnected::new(100, 10)));

    assert_eq!(model.get_layers().len(), 2);
}

#[test]
#[should_panic]
fn add_layers_incompatible() {
    let mut model: Sequential = Sequential::new();

    model.add(Box::new(FullyConnected::new(10, 100)));
    model.add(Box::new(FullyConnected::new(140, 10)));
}
