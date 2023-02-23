#![allow(non_snake_case)]
mod lib;
use std::env;

use lib::{network::Network, activations::SIGMOID};


// 0, 0  -> 0
// 1, 0  -> 1
// 0, 1  -> 1
// 1, 1  -> 0

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let inputs = vec![
        vec![0.0,  0.0],
        vec![1.0,  0.0],
        vec![0.0,  1.0],
        vec![1.0,  1.0],
    ];
    let targets = vec![
        vec![0.0],
        vec![1.0],
        vec![1.0],
        vec![0.0],
    ];
    let mut network = Network::new(
        vec![2, 3, 1], 
        0.1, 
        SIGMOID,
    );
    network.train(inputs, targets, 500000);
    println!("0, 0 -> {:?}", network.feedForward(vec![0.0, 0.0]));
    println!("1, 0 -> {:?}", network.feedForward(vec![1.0, 0.0]));
    println!("0, 1 -> {:?}", network.feedForward(vec![0.0, 1.0]));
    println!("1, 1 -> {:?}", network.feedForward(vec![1.0, 1.0]));
}