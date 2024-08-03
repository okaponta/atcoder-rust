#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut dp = vec![vec![-1 << 30; 3]; n + 1];
    dp[0][0] = 0i64;
    dp[0][1] = 0;
    dp[0][2] = 0;
    for i in 0..n {
        if s[i] == 'R' {
            dp[i + 1][2] = dp[i][0].max(dp[i][1]) + 1;
            dp[i + 1][0] = dp[i][1].max(dp[i][2]);
        } else if s[i] == 'S' {
            dp[i + 1][0] = dp[i][1].max(dp[i][2]) + 1;
            dp[i + 1][1] = dp[i][0].max(dp[i][2]);
        } else {
            dp[i + 1][1] = dp[i][0].max(dp[i][2]) + 1;
            dp[i + 1][2] = dp[i][0].max(dp[i][1]);
        }
    }
    let ans = dp[n][0].max(dp[n][1]).max(dp[n][2]);
    println!("{}", ans);
}
