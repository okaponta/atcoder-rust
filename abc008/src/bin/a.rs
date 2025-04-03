#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:usize,
        t:usize,
    }
    println!("{}", t - s + 1);
}
