#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut dp = vec![-1 << 60; 2];
    dp[1] = 0;
    for a in a {
        let mut next = dp.clone();
        next[0] = next[0].max(dp[1] + a);
        next[1] = next[1].max(dp[0] + a * 2);
        dp = next;
    }
    println!("{}", dp[0].max(dp[1]));
}
