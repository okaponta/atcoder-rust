#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        x:usize,
    }
    println!("{}", if a.contains(&x) { "Yes" } else { "No" });
}
