#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut ab:[(usize,usize);n],
    }
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    ab.sort_by(|a, b| ((a.0 - 1) * b.1).cmp(&((b.0 - 1) * a.1)));
    for (a, b) in ab {
        for i in (1..=k).rev() {
            dp[i] = dp[i].max(dp[i - 1] * a + b);
        }
    }
    println!("{}", dp[k]);
}
