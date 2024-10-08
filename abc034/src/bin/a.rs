#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        x:usize,
        y:usize,
    }
    println!("{}", if x < y { "Better" } else { "Worse" });
}
