#![allow(non_snake_case)]
mod lib;
use std::env;

use lib::{network::Network};

use crate::lib::activations::{SIGMOID, RELU, TANH};


// 0, 0  -> 0
// 1, 0  -> 1
// 0, 1  -> 1
// 1, 1  -> 0



fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let P22: Vec<f64> = vec![  
        0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let P33: Vec<f64> = vec![  
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let P36: Vec<f64> = vec![  
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0];
    let P66: Vec<f64> = vec![  
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0];
    let P77: Vec<f64> = vec![  
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0];
    let P11: Vec<f64> = vec![  
        1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

        let P62: Vec<f64> = vec![  
            0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let P44: Vec<f64> = vec![  
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let P57: Vec<f64> = vec![  
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0];
        let P27: Vec<f64> = vec![  
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        let P53: Vec<f64> = vec![  
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
                                
    let inputs = vec![P22.clone(), P33.clone(), P36.clone(), P66.clone(), P77.clone(), P11.clone(), P62.clone(), P44.clone(), P57.clone()];
    let targets = vec![
        vec![0.22],
        vec![0.33],
        vec![0.36],
        vec![0.66],
        vec![0.77],
        vec![0.11],
        vec![0.62],
        vec![0.44],
        vec![0.57],
    ];
    let mut network = Network::new(
        vec![49, 49, 49, 49, 1], 
        0.5, 
        SIGMOID,
        // TANH,
        // RELU,
    );
    network.train(inputs, targets, 50000);
    println!("P22 -> {:?}", network.feedForward(P22));
    println!("P33 -> {:?}", network.feedForward(P33));
    println!("P36 -> {:?}", network.feedForward(P36));
    println!("P66 -> {:?}", network.feedForward(P66));
    println!("P77 -> {:?}", network.feedForward(P77));
    println!("P11 -> {:?}", network.feedForward(P11));
    println!("P62 -> {:?}", network.feedForward(P62));
    println!("P44 -> {:?}", network.feedForward(P44));
    println!("P57 -> {:?}", network.feedForward(P57));
    println!("P27 -> {:?}", network.feedForward(P27));
    println!("P53 -> {:?}", network.feedForward(P53));
}




    // let A1: Vec<f64> = vec![  
    //     0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    //     0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    //     0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    // let A2: Vec<f64> = vec![  
    //     0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    //     0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    //     0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    // let A3: Vec<f64> = vec![  
    //     0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
    //     0.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0,
    //     0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
    //     1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    // let B1: Vec<f64> = vec![  
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0];
    // let B2: Vec<f64> = vec![  
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0];
    // let B3: Vec<f64> = vec![  
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    //     0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0];

    
    // let inputs = vec![
    //     vec![0.0,  0.0],
    //     vec![1.0,  0.0],
    //     vec![0.0,  1.0],
    //     vec![1.0,  1.0],
    // ];