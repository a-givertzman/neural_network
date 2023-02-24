#![allow(non_snake_case)]
mod lib;
use std::{env, process::exit};
use rand::{thread_rng, Rng};

use lib::{network::Network};
use crate::lib::{activations::{SIGMOID, RELU, TANH}, matrix::Matrix};

struct TrainMatrix {
    data: Vec<f64>,
    x: usize,
    y: usize,
}

fn trainMatrix(rows: usize, cols: usize, debug: bool) -> TrainMatrix {
    let mut rng = thread_rng();
    let x = (rng.gen::<f64>() * 6.0 + 0.0).round() as usize;
    let y = (rng.gen::<f64>() * 6.0 + 0.0).round() as usize;
    let mut rect = vec![vec![0.0; cols]; rows];
    rect[y][x] = 1.0;
    if y > 0 {rect[y -1][x] = 1.0;}
    if y < (rows -1) {rect[y +1][x] = 1.0;}
    if x > 0 {rect[y][x -1] = 1.0;}
    if x < (cols -1) {rect[y][x +1] = 1.0;}
    println!("{}:{}", x, y);
    if debug {
        for i in 0..rows {
            let ivec: Vec<usize> = rect[i].clone().into_iter().map(|v| ((v as f32).round()) as usize).collect();
            println!("{}:{} | {:?}", x, y, ivec);
        }
    }
    let mut res = vec![];
    for i in 0..rows {
        res.append(&mut rect[i]);
    }
    println!("{}:{}({}) | {:?}", x, y, res.len(), res);
    if res.len() > rows * cols {
        panic!("Wrong length ({:?}) of input value:\n{:?}", res.len(), res);
    }
    TrainMatrix{data: res, x, y}
}


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let learnEpochs = 100;
    let learnDataCount = 100;
    let rows = 6;
    let cols = 8;
    let dimension = rows * cols;

    let mut inputs = vec![];
    let mut targets = vec![];
    for i in 0..learnDataCount {
        let tMatrix = trainMatrix(rows, cols, false);
        inputs.push(tMatrix.data);
        targets.push(vec![(tMatrix.y as f64) * 0.1 + (tMatrix.x as f64) * 0.01]);
    }

                                
    let mut network = Network::new(
        vec![dimension, dimension * 2, dimension * 2, dimension, 1], 
        0.4, 
        SIGMOID,
        // TANH,
        // RELU,
    );
    network.train(inputs.clone(), targets.clone(), learnEpochs);
    for i in 0..inputs.len() {
        let tMatrix = trainMatrix(rows, cols, false);
        let target = vec![(tMatrix.y as f64) * 0.1 + (tMatrix.x as f64) * 0.01];
        println!("C{:?} -> {:?}", target, network.feedForward(tMatrix.data));
    }
}
