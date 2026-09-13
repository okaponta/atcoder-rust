#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*,std::vec};
use std::println;

fn main() {
    input! {
        n:usize,
        s:[String;n],
    }
    let mut map = HashMap::new();
    for s in s {
        *map.entry(s.to_ascii_lowercase()).or_insert(0) += 1;
    }
    println!("{}", map.values().max().unwrap());
}
