#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        r:usize,
        x:usize,
    }
    if x == 1 {
        println!("{}", if 1600 <= r && r < 3000 { "Yes" } else { "No" });
    } else {
        println!("{}", if 1200 <= r && r < 2400 { "Yes" } else { "No" });
    }
}
