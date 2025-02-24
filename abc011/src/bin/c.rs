#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        ng:[usize;3],
    }
    let mut dp = vec![200; n + 5];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..4 {
            if ng.contains(&(i + j)) {
                continue;
            }
            dp[i + j] = dp[i + j].min(dp[i] + 1);
        }
    }
    if dp[n] <= 100 {
        println!("YES");
    } else {
        println!("NO");
    }
}
