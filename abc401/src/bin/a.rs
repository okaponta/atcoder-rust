#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:usize,
    }
    println!(
        "{}",
        if 200 <= s && s < 300 {
            "Success"
        } else {
            "Failure"
        }
    );
}
