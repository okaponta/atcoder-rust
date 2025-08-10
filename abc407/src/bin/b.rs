#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        x:usize,
        y:usize,
    }
    let mut count = 0;
    for i in 1..7 {
        for j in 1..7 {
            if x <= i + j || y <= abs(i, j) {
                count += 1;
            }
        }
    }
    println!("{}", count as f64 / 36.0);
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
