#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*,std::vec};
use std::println;

fn main() {
    input! {
        a:usize,
        b:usize,
    }
    if a + b == 9 {
        println!("Nine");
        return;
    }
    if a + 9 == b || b + 9 == a {
        println!("Nine");
        return;
    }
    if a * b == 9 {
        println!("Nine");
        return;
    }
    if b * 9 == a || a * 9 == b {
        println!("Nine");
        return;
    }
    println!("Nein");
}
