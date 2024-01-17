
///
/// Abstract neural network model.
///
/// All models inherit from model, including Sequential.
/// A model has a set of layers, which convey information between each other.
///
pub trait Model {

    fn forward();
}
