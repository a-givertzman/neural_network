mod lib;
use crate::lib::matrix::Matrix;

fn main() {
    let m = Matrix {rows: 4, cols: 4, data: vec![vec![0.0; 4]; 4]};
    println!("Matrix 4x4: {:?}", m);
    let mr = Matrix::rendom(4, 4);
    println!("Matrix ranom 4x4: {:?}", mr);
}
