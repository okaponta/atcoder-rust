#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use std::println;

fn main() {
    input! {
        n:usize,
    }
    for i in 1..=n {
        if i % 3 == 0 {
            println!("Fizz");
        } else {
            println!("{i}")
        }
    }
}
