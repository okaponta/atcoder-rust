#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    a.sort();
    a.dedup();
    a.reverse();
    println!("{}", a[1]);
}
