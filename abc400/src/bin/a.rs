#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        a:usize,
    }
    if 400 % a != 0 {
        println!("-1");
    } else {
        println!("{}", 400 / a);
    }
}
