#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    println!("{}", (2 * a + b) / (2 * b))
}
