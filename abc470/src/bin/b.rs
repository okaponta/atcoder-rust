#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*,std::vec};
use std::println;

fn main() {
    input! {
        n:usize,
        c:[usize;n],
    }
    let mut count = vec![0; 101];
    for c in c {
        count[c] += 1;
    }
    let max = count.into_iter().max().unwrap();
    println!("{}", n - max);
}
