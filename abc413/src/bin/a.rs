#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
    }
    println!(
        "{}",
        if a.into_iter().sum::<usize>() <= m {
            "Yes"
        } else {
            "No"
        }
    );
}
