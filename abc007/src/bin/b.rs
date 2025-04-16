#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        a:String,
    }
    if a == "a" {
        println!("-1");
        return;
    }
    println!("a");
}
