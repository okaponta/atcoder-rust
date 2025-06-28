#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        ab:[(usize,usize);n],
    }
    println!("{}", ab.into_iter().filter(|(a, b)| a < b).count());
}
