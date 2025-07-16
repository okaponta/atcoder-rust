#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
    }
    if c < a || a == c && d < b {
        println!("Yes")
    } else {
        println!("No")
    }
}
