use super::{matrix::Matrix, activations::Activation};

pub struct Network<'a> {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    learningRate: f64,
    activation: Activation<'a>,
}

impl Network<'_> {
    ///
    /// creates new instance of the Network
    pub fn new<'a>(layers: Vec<usize>, learningRate: f64, activation: Activation<'a>) -> Network {
        let mut weights = vec![];
        let mut biases = vec![];
        for i in 0..(layers.len() -1) {
            weights.push(Matrix::random(layers[i] + 1, layers[i]));
            biases.push(Matrix::random(layers[i] + 1, 1));
        }
        Network { 
            layers, 
            weights, 
            biases, 
            data: vec![],
            learningRate,
            activation,
        }
    }
    ///
    pub fn feedForward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Invalid number of inputs")
        }
        let mut current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];
        for i in 0..self.layers.len() -1 {
            current = self.weights[i]
                .multiply(&current)
                .add(&self.biases[i])
                .map(self.activation.function);
            self.data.push(current.clone());
        }
        current.data[0].to_owned()
    }
    ///
    pub fn backPropogate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != self.layers[self.layers.len() -1] {
            panic!("Invalid number of targets");
        }
        let mut parsed = Matrix::from(vec![outputs]);
        let mut errors = Matrix::from(vec![targets]).subtract(&parsed);
        let mut gradients = parsed.map(self.activation.derivative);
        for i in (0..self.layers.len() -1).rev() {
            gradients = gradients.dotMultiply(&errors).map(&|x| x * self.learningRate);
            self.weights[i] = self.weights[i].add(&gradients.multiply(&self.data[i].transpose()));
            self.biases[i] = self.biases[i].add(&gradients);
            errors = self.weights[i].transpose().multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }
}