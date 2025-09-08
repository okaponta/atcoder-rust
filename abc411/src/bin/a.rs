#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        p:Chars,
        l:usize,
    }
    println!("{}", if l <= p.len() { "Yes" } else { "No" });
}
