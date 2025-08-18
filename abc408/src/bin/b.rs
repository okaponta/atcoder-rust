#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.dedup();
    println!("{}", a.len());
    println!("{}", a.iter().join(" "));
}
