#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        x:usize,
    }
    let ans = (x - 1).min(n - x);
    println!("{}", ans);
}
