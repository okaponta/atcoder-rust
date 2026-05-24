#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        x:usize,
    }
    let mut s = "HelloWorld".to_string().chars().collect_vec();
    s.remove(x - 1);
    println!("{}", s.iter().join(""));
}
