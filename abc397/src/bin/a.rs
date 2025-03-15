#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:String,
    }
    if s < "37.5".to_string() {
        println!("3");
    } else if s < "38.0".to_string() {
        println!("2");
    } else {
        println!("1");
    }
}
