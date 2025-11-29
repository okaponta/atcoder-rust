#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        w:usize,
        b:usize,
    }
    println!("{}", (w * 1000 + b) / b);
}
