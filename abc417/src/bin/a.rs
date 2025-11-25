#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        _n:usize,
        a:usize,
        b:usize,
        mut s:Chars,
    }
    for _ in 0..a {
        s.remove(0);
    }
    for _ in 0..b {
        s.pop();
    }
    println!("{}", s.iter().join(""));
}
