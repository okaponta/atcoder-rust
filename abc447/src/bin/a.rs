#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    println!("{}", if m <= (n + 1) / 2 { "Yes" } else { "No" });
}
