#![allow(non_snake_case)]
use rand::Rng;



#[derive(Debug, Clone)]
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
    /// creates Matrix from Vec<Vec<f64>>
    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        Matrix { 
            rows: data.len(), 
            cols: data.first().unwrap().len(), 
            data,
        }
    }
    ///
    /// matrix filled with random values
    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        Matrix{
            rows, 
            cols,
            data: (0..rows).map(|_| {
                (0..cols).map(|_| {
                    // let res = (rowIndex as f64) + (colIndex as f64) / 10.0;
                    // println!("rowIndex: {}, colIndex: {} | {:?}", rowIndex, colIndex, res);
                    rng.gen::<f64>() * 2.0 -1.0
                }).collect()
            }).collect(),
        }
    }
    ///
    /// multyplies a self by other
    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Inposible to multiply by matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, other.cols);
        for rowIndex in 0..self.rows {
            for colIndex in 0..other.cols {
                let mut sum = 0.0;
                for index  in 0..self.cols {
                    sum += self.data[rowIndex][index] * other.data[index][colIndex];
                }
                res.data[rowIndex][colIndex] = sum;
            }
        }
        res
    }
    ///
    /// adds a self and other
    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.cols || self.rows != other.rows {
            panic!("Inposible to add matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, other.cols);
        for rowIndex in 0..self.rows {
            for colIndex in 0..self.cols {
                res.data[rowIndex][colIndex] = self.data[rowIndex][colIndex] + other.data[rowIndex][colIndex];
            }
        }
        res
    }
    ///
    /// dot multiplies self by other matrix
    pub fn dotMultiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.cols || self.rows != other.rows {
            panic!("Inposible to dot multiply by matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, other.cols);
        for rowIndex in 0..self.rows {
            for colIndex in 0..self.cols {
                res.data[rowIndex][colIndex] = self.data[rowIndex][colIndex] * other.data[rowIndex][colIndex];
            }
        }
        res
    }
    ///
    /// subtracts a self and other
    pub fn subtract(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.cols || self.rows != other.rows {
            panic!("Inposible to subtract matrix of incorrect dimesions" );
        }
        let mut res = Matrix::zeros(self.rows, other.cols);
        for rowIndex in 0..self.rows {
            for colIndex in 0..self.cols {
                res.data[rowIndex][colIndex] = self.data[rowIndex][colIndex] - other.data[rowIndex][colIndex];
            }
        }
        res
    }
    ///
    /// Maps all element of a matrix
    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        Matrix::from(
            (self.data).clone().into_iter().map(|row| {
                row.into_iter().map(|value| {
                    function(value)
                }).collect()
            }).collect(),
        )
    }
    ///
    /// transposes a matrix
    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }
        res
    }
}