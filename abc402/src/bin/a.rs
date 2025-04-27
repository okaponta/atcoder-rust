#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        mut s:Chars,
    }
    println!("{}", s.iter().filter(|c| c.is_uppercase()).join(""));
}
