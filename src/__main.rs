#![allow(non_snake_case)]

mod lib;
use crate::lib::matrix::Matrix;
// use color_backtrace::{default_output_stream, BacktracePrinter};

fn main() {
    // better_panic::install();
    color_backtrace::install();
    let m = Matrix {rows: 4, cols: 4, data: vec![vec![0.0; 4]; 4]};
    println!("Matrix 4x4: {:?}", m);

    let mr = Matrix::zeros(4, 4);
    println!("Matrix zeroz 4x4: {:?}", mr);

    let mr = Matrix::random(4, 4);
    println!("Matrix ranom 4x4: {:?}", mr);


    {
        let tar = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![6.0, 10.0, 14.0], 
                vec![12.0, 20.0, 28.0], 
                vec![18.0, 30.0, 42.0]
            ],
        };
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
        assert!(res.data == tar.data, "multiply test failed");
    }


    {
        let tar = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.111, 1.212, 1.313], 
                vec![2.121, 2.222, 2.323], 
                vec![3.131, 3.232, 3.333]
            ],
        };
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
                vec![0.011, 0.012, 0.013], 
                vec![0.021, 0.022, 0.023], 
                vec![0.031, 0.032, 0.033]
            ],
        };
        let res = m1.add(&m2);
        println!("add res: {:?}", res);
        for i in 0..tar.rows {
            for j in 0..tar.cols {
                let resValue = (res.data[i][j] * 1000.0).round() / 1000.0;
                assert!(resValue == tar.data[i][j], "add test failed actual {:?} != target {:?}", resValue, tar.data[i][j]);
            }
        }
    }

    {
        let tar = Matrix {
            rows: 3,
            cols: 3,
            data: vec![
                vec![11.0, 12.0, 13.0], 
                vec![21.0, 22.0, 23.0], 
                vec![31.0, 32.0, 33.0]
            ],
        };
        let mut m1 = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![10.0, 10.0, 10.0], 
                vec![10.0, 10.0, 10.0], 
                vec![10.0, 10.0, 10.0]
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
        assert!(res.data == tar.data, "dotMultiply test failed");
    }

    {
        let tar =  Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.1, 1.2, 1.3], 
                vec![2.1, 2.2, 2.3], 
                vec![3.1, 3.2, 3.3]
            ],
        };
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
        for i in 0..tar.rows {
            for j in 0..tar.cols {
                let resValue = (res.data[i][j] * 10.0).round() / 10.0;
                assert!(resValue == tar.data[i][j], "subtract test failed actual {:?} != target {:?}", resValue, tar.data[i][j]);
            }
        }
    }

    {
        let factor = 2.0;
        let tar = Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.1 + factor, 1.2 + factor, 1.3 + factor], 
                vec![2.1 + factor, 2.2 + factor, 2.3 + factor], 
                vec![3.1 + factor, 3.2 + factor, 3.3 + factor]
            ],
        };
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
            value + factor
        });
        println!("map res: {:?}", res);
        assert!(res.data == tar.data, "map test failed");
    }

    {
        let tar =  Matrix{
            rows: 3,
            cols: 3,
            data: vec![
                vec![1.1, 2.1, 3.1], 
                vec![1.2, 2.2, 3.2], 
                vec![1.3, 2.3, 3.3]
            ],
        };
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
        assert!(res.data == tar.data, "transpose test failed");
    }

}
