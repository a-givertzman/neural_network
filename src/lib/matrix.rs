use rand::Rng;

#[derive(Debug)]
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
    pub fn rendom(rows: usize, cols: usize) -> Matrix {
        let mut rng = rand::thread_rng();
        let mut m = Matrix::zeros(rows, cols);
        for rowIndex in 0..(rows -1) {
            for colIndex in 0..(cols -1) {
                m.data[rowIndex][colIndex] = rng.gen::<f64>() * 2.0 -1.0;
            }
        }
        m
    }
}