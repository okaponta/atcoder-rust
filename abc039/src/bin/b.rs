#[allow(unused)]
use itertools::*;
use num_integer::Roots;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", n.sqrt().sqrt());
}
