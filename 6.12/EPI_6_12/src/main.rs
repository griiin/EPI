extern crate rand;
use rand::distributions::{IndependentSample, Range};

fn generate_rand(from: usize, to: usize) -> usize {
    let between = Range::new(from, to);
    let mut rng = rand::thread_rng();
    between.ind_sample(&mut rng)
}

fn extract(v: &mut Vec<i32>, size: usize) -> Vec<i32> {
    let mut result:Vec<i32> = Vec::new();
    for i in 0..size {
        let rand = generate_rand(0, v.len() - 1 - i);
        result.push(v.remove(rand))
    }
    result
}

fn main() {
    let mut v = vec![1,2,3,4,5,6,7,8,9,10,11];
    println!("{:?}", extract(&mut v, 3));
}
