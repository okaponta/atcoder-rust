#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut dp = vec![0; n];
    dp[1] = abs(a[0], a[1]);
    for i in 2..n {
        dp[i] = (dp[i - 1] + abs(a[i - 1], a[i])).min(dp[i - 2] + abs(a[i - 2], a[i]));
    }
    println!("{}", dp[n - 1]);
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
