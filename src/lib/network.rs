use super::{matrix::Matrix, activations::Activation};

const _DEBUG: bool = false;

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
    pub fn new<'a>(
        layers: Vec<usize>, 
        learningRate: f64, 
        activation: Activation<'a>,
    ) -> Network<'a> {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            weights.push(Matrix::zeros(layers[i + 1], layers[i]));
            biases.push(Matrix::zeros(layers[i + 1], 1));
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
        if _DEBUG {println!("[Network.feedForward] current: {:?}", current);}
        if _DEBUG {println!("[Network.feedForward] current.transpose(): {:?}", current.transpose());}
        if _DEBUG {println!("[Network.feedForward] current.transpose().data[0]: {:?}", current.transpose().data[0]);}
        current.transpose().data[0].to_owned()
    }
    ///
    /// 
    pub fn backPropogate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
        if targets.len() != self.layers[self.layers.len() -1] {
            panic!("Invalid number of targets");
        }
        // if _DEBUG {println!("[Network.backPropogate] outputs: {:?}", outputs);}
        // if _DEBUG {println!("[Network.backPropogate] targets: {:?}", targets);}
        let parsed = Matrix::from(vec![outputs]).transpose();
        if _DEBUG {println!("[Network.backPropogate] parsed: {:?}", parsed.clone());}
        let errorsTr = Matrix::from(vec![targets]).transpose();
        if _DEBUG {println!("[Network.backPropogate] errorsTr: {:?}", errorsTr.clone());}
        let mut errors = errorsTr.subtract(&parsed);
        let mut gradients = parsed.map(self.activation.derivative);
        for i in (0..self.layers.len() -1).rev() {
            gradients = gradients
                .dotMultiply(&errors)
                .map(&|x| x * self.learningRate);
            self.weights[i] = self.weights[i]
                .add(
                    &gradients.multiply(&self.data[i].transpose()))
                ;
            self.biases[i] = self.biases[i].add(&gradients);
            errors = self.weights[i].transpose().multiply(&errors);
            gradients = self.data[i].map(self.activation.derivative);
        }
    }
    ///
    /// use to lern a model
    pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u16) {
        for i in 1..=epochs {
            if i < 100 || i % (epochs / 100) == 0 {
                if _DEBUG {println!("[Network.train] Epoch {:?} of {:?}", i, epochs);}
            }
            for j in 0..inputs.len() {
                if _DEBUG {println!("[Network.train] input {:?}: {:?}", j, inputs[j].clone());}
                let outputs = self.feedForward(inputs[j].clone());
                if _DEBUG {println!("[Network.train] outputs: {:?}", outputs.clone());}
                if _DEBUG {println!("[Network.train] targets: {:?}", targets.clone());}
                if _DEBUG {println!("[Network.train] targets[{}]: {:?}", j, targets[j].clone());}
                self.backPropogate(outputs, targets[j].clone());
            }
        }
    }
}