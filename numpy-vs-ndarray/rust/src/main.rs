use ndarray::prelude::*;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use ndarray_stats::QuantileExt;
use std::time::Instant;

fn argmax(shape: (usize, usize, usize)) -> u128 {
    let a = Array::random(shape, Uniform::new(0., 1.));
    let start = Instant::now();
    a.argmax().unwrap();
    start.elapsed().as_micros()
}

fn max(shape: (usize, usize, usize)) -> u128 {
    let a = Array::random(shape, Uniform::new(0., 1.));
    let start = Instant::now();
    a.max().unwrap();
    start.elapsed().as_micros()
}

fn shape_test(shape: (usize, usize, usize)) {
    let mut times: Vec<u128> = vec![];
    println!("{:?}", shape);
    for _ in 1..1000 {
        times.push(max(shape));
    }
    // calculate average
    let mut sum: u128 = 0;
    for x in &times {
        sum += x;
    }
    let avg = sum as f32 / times.len() as f32;
    println!("Average time for max: {}µs", avg);

    let mut times: Vec<u128> = vec![];
    for _ in 1..1000 {
        times.push(argmax(shape));
    }
    // calculate average
    let mut sum: u128 = 0;
    for x in &times {
        sum += x;
    }
    let avg = sum as f32 / times.len() as f32;
    println!("Average time for argmax: {}µs", avg);
}

fn main() {
    //     shape_test((1, 512, 512));
    //     shape_test((1, 128, 128));
    // shape_test((1, 12, 128));
    shape_test((1, 2, 128));
    // shape_test((1, 1, 128));
}
