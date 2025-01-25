#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        w:usize,
        n:usize,
        k:usize,
        ab:[(usize,usize);n],
    }
    let mut ans = 0;
    let mut dp = vec![vec![0; w + 1]; k + 1];
    for (a, b) in ab {
        for j in (0..k).into_iter().rev() {
            for l in 0..w {
                if l + a <= w {
                    dp[j + 1][l + a] = dp[j + 1][l + a].max(dp[j][l] + b);
                    ans = ans.max(dp[j + 1][l + a]);
                }
            }
        }
    }
    println!("{}", ans);
}
