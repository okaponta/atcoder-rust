#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }
    println!("{}", 2 * (a * b + b * c + c * a));
}
