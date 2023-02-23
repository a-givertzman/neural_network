#![allow(non_snake_case)]
use rand::{thread_rng, Rng};
use std::fmt::{Debug, Formatter, Result};


const _DEBUG: bool = false;

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    ///
    /// matrix filled with 0.0
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    ///
    /// matrix filled with random values
    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
        res
    }
    ///
    /// creates Matrix from Vec<Vec<f64>>
    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix { 
            rows: data.len(), 
            cols: data[0].len(), 
            data,
        }
    }    
    ///
    /// multyplies a self by other
    pub fn multiply(&self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Inposible to multiply by matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }
                res.data[i][j] = sum;
            }
        }
        res
    }
    ///
    /// adds a self and other
    pub fn add(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Inposible to add matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        res
    }
    ///
    /// dot multiplies self by other matrix
    pub fn dotMultiply(&self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Inposible to dot multiply by matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }
        res
    }
    ///
    /// subtracts a self and other
    pub fn subtract(&self, other: &Matrix) -> Matrix {
        if _DEBUG {println!("[Matrix.subtract] self: {:?}", self.clone());}
        if _DEBUG {println!("[Matrix.subtract] other: {:?}", other.clone());}

        if self.rows != other.rows || self.cols != other.cols {
            panic!("Inposible to subtract matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        res
    }
    ///
    /// Maps all element of a matrix
    pub fn map(&self, function: &dyn Fn(f64) -> f64) -> Matrix {
        Matrix::from(
            (self.data).clone().into_iter().map(|row| {
                    row.into_iter().map(|value| {
                    function(value)
                }).collect()
            }).collect(),
        )
    }
    ///
    /// returns a new matrix transposed from self
    pub fn transpose(&self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }
}

impl Debug for Matrix {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(
			f,
			"Matrix {{\n{}\n}}",
			(&self.data)
				.into_iter()
				.map(|row| "  ".to_string()
					+ &row
						.into_iter()
						.map(|value| value.to_string())
						.collect::<Vec<String>>()
						.join(" "))
				.collect::<Vec<String>>()
				.join("\n")
		)
	}
}