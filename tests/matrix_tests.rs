use neujal::linalg::matrix::Matrix;

#[test]
fn create_matrix() {
    let mut m1: Matrix = Matrix::new((2, 2), false);
    m1[0][0] = 1.0;
    m1[0][1] = 2.0;
    m1[1][0] = 3.0;
    m1[1][1] = 4.0;
}

#[test]
fn display_matrix() {
    let mut matrix: Matrix = Matrix::new((2, 2), false);
    matrix[0][0] = 1.0;
    matrix[0][1] = 2.0;
    matrix[1][0] = 3.0;
    matrix[1][1] = 4.0;

    println!("{}", matrix);

    let s: String = "1.00\t2.00\t\n3.00\t4.00\t\n".to_string();

    assert_eq!(matrix.to_string(), s);
}

#[test]
fn multiply_matrices() {
    let mut m1: Matrix = Matrix::new((2, 2), false);
    m1[0][0] = 1.0;
    m1[0][1] = 2.0;
    m1[1][0] = 3.0;
    m1[1][1] = 4.0;

    let mut m2: Matrix = Matrix::new((2, 2), false);
    m2[0][0] = 4.0;
    m2[0][1] = 3.0;
    m2[1][0] = 2.0;
    m2[1][1] = 1.0;

    let mut expected: Matrix = Matrix::new((2, 2), false);
    expected[0][0] = 8.0;
    expected[0][1] = 5.0;
    expected[1][0] = 20.0;
    expected[1][1] = 13.0;

    assert!(&m1 * &m2 == expected)
}

#[test]
fn subtract_matrices() {
    let mut m1: Matrix = Matrix::new((2, 2), false);
    m1[0][0] = 1.0;
    m1[0][1] = 2.0;
    m1[1][0] = 3.0;
    m1[1][1] = 4.0;

    let mut m2: Matrix = Matrix::new((2, 2), false);
    m2[0][0] = 4.0;
    m2[0][1] = 3.0;
    m2[1][0] = 2.0;
    m2[1][1] = 1.0;

    let mut expected: Matrix = Matrix::new((2, 2), false);
    expected[0][0] = -3.0;
    expected[0][1] = -1.0;
    expected[1][0] = 1.0;
    expected[1][1] = 3.0;

    assert!(&m1-&m2 == expected);
}

#[test]
fn sum_matrices() {
    let mut m1: Matrix = Matrix::new((2, 2), false);
    m1[0][0] = 1.0;
    m1[0][1] = 2.0;
    m1[1][0] = 3.0;
    m1[1][1] = 4.0;

    let mut m2: Matrix = Matrix::new((2, 2), false);
    m2[0][0] = 4.0;
    m2[0][1] = 3.0;
    m2[1][0] = 2.0;
    m2[1][1] = 1.0;

    let mut expected: Matrix = Matrix::new((2, 2), false);
    expected[0][0] = 5.0;
    expected[0][1] = 5.0;
    expected[1][0] = 5.0;
    expected[1][1] = 5.0;

    assert!(&m1+&m2 == expected);
}

#[test]
#[should_panic]
fn subtract_incompatible() {
    let mut m1: Matrix = Matrix::new((1, 2), false);
    m1[0][0] = 1.0;
    m1[0][1] = 2.0;

    let mut m2: Matrix = Matrix::new((2, 2), false);
    m2[0][0] = 4.0;
    m2[0][1] = 5.0;
    m2[1][0] = 2.0;
    m2[1][1] = 1.0;

    let _out: Matrix = &m1 - &m2;
}

#[test]
#[should_panic]
fn multiply_incompatible() {
    let mut m1: Matrix = Matrix::new((2, 1), false);
    m1[0][0] = 1.0;
    m1[1][0] = 2.0;

    let mut m2: Matrix = Matrix::new((2, 2), false);
    m2[0][0] = 4.0;
    m2[0][1] = 5.0;
    m2[1][0] = 2.0;
    m2[1][1] = 1.0;

    let _out: Matrix = &m1 * &m2;
}