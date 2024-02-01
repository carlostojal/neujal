use neujal::linalg::matrix::Matrix;

pub fn main() {

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

    let res: Matrix = &m1 * &m2;

    println!("{} * \n{} = \n{}", m1, m2, res);

}