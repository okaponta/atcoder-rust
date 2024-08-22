#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        h1:usize,
        w1:usize,
        h2:usize,
        w2:usize,
    }
    println!(
        "{}",
        if h1 == h2 || h1 == w2 || w1 == h2 || w1 == w2 {
            "YES"
        } else {
            "NO"
        }
    );
}
