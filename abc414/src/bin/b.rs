#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        cl:[(char,u128);n],
    }
    if 100 < cl.iter().map(|(_, l)| *l).sum::<u128>() {
        println!("Too Long");
        return;
    }
    for (c, l) in cl {
        for _ in 0..l {
            print!("{c}");
        }
    }
    println!();
}
