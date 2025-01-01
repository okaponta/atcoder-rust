#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut ans = 1;
    for i in 1..n {
        let mut tmp = 1;
        let mut dp = vec![1; n];
        for j in 0..n {
            if i <= j && h[j] == h[j - i] {
                dp[j] = dp[j - i] + 1;
            }
            tmp = tmp.max(dp[j]);
        }
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
