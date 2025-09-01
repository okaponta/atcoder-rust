#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        k:usize,
    }
    println!("{}", a.into_iter().filter(|i| &k <= i).count());
}
