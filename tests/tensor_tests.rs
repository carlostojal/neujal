use neujal::linalg::tensor::Tensor;

#[test]
fn test_create_tensor() {
    let shape: (usize,usize,usize) = (2,3,2);
    let tensor: Tensor = Tensor::new(shape, false);

    assert_eq!(tensor.get_shape(), shape);
    assert_eq!(tensor.get_values().len(), shape.0); // x dimension
    assert_eq!(tensor.get_values()[0].len(), shape.1); // y dimension
    assert_eq!(tensor.get_values()[0][0].len(), shape.2); // z dimension

}