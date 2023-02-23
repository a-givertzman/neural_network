#![allow(non_snake_case)]

mod lib;
use crate::lib::matrix::Matrix;

fn main() {
    let m = Matrix {rows: 4, cols: 4, data: vec![vec![0.0; 4]; 4]};
    println!("Matrix 4x4: {:?}", m);

    let mr = Matrix::zeros(4, 4);
    println!("Matrix zeroz 4x4: {:?}", mr);

    let mr = Matrix::rendom(4, 4);
    println!("Matrix ranom 4x4: {:?}", mr);

    let rows = 4;
    let cols = 4;
    let res: Vec<Vec<f64>> = (0..rows).map(|rowIndex| {
        (0..cols).map(|colIndex| {
            let res = (rowIndex as f64) + (colIndex as f64) / 10.0;
            println!("rowIndex: {}, colIndex: {} | {:?}", rowIndex, colIndex, res);
            res
        }).collect()
    }).collect();
    println!("res: {:?}", res);

    {
        let mut m1 = Matrix{
            rows: 3,
            cols: 1,
            data: vec![vec![2.0], vec![4.0], vec![6.0]],
        };
        let m2 = Matrix {
            rows: 1,
            cols: 3,
            data: vec![vec![3.0, 5.0, 7.0]],
        };
        let res = m1.multiply(&m2);
        println!("mul res: {:?}", res);
    }


    {
        let mut m1 = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.1, 1.2, 1.3], 
                vec![2.1, 2.2, 2.3], 
                vec![3.1, 3.2, 3.3]
            ],
        };
        let m2 = Matrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![0.01, 0.02, 0.03], 
                vec![0.01, 0.02, 0.03], 
                vec![0.01, 0.02, 0.03]
            ],
        };
        let res = m1.add(&m2);
        println!("add res: {:?}", res);
    }

    {
        let mut m1 = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.0, 1.0, 1.0], 
                vec![1.0, 1.0, 1.0], 
                vec![1.0, 1.0, 1.0]
            ],
        };
        let m2 = Matrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.1, 1.2, 1.3], 
                vec![2.1, 2.2, 2.3], 
                vec![3.1, 3.2, 3.3]
            ],
        };
        let res = m1.dotMultiply(&m2);
        println!("dotMultiply res: {:?}", res);
    }

    {
        let mut m1 = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.111, 1.212, 1.313], 
                vec![2.121, 2.222, 2.323], 
                vec![3.131, 3.232, 3.333]
            ],
        };
        let m2 = Matrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![0.011, 0.012, 0.013], 
                vec![0.021, 0.022, 0.023], 
                vec![0.031, 0.032, 0.033]
            ],
        };
        let res = m1.subtract(&m2);
        println!("subtract res: {:?}", res);
    }

    {
        let mut m1 = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.1, 1.2, 1.3], 
                vec![2.1, 2.2, 2.3], 
                vec![3.1, 3.2, 3.3]
            ],
        };
        let res = m1.map(&|value| {
            value + 1.0
        });
        println!("subtract res: {:?}", res);
    }

    {
        let mut m1 = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.1, 1.2, 1.3], 
                vec![2.1, 2.2, 2.3], 
                vec![3.1, 3.2, 3.3]
            ],
        };
        let res = m1.transpose();
        println!("transpose res: {:?}", res);
    }

}
