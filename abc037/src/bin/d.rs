#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[usize;w];h],
    }
    let mut ord = vec![];
    for i in 0..h {
        for j in 0..w {
            ord.push((a[i][j], i, j));
        }
    }
    ord.sort();
    let mut dp = vec![vec![1; w]; h];
    for (i, hi, wi) in ord {
        for (dx, dy) in vec![(!0, 0), (0, 1), (0, !0), (1, 0)] {
            let nh = hi.wrapping_add(dx);
            let nw = wi.wrapping_add(dy);
            if h <= nh || w <= nw {
                continue;
            }
            if a[nh][nw] < i {
                dp[hi][wi] += dp[nh][nw];
                dp[hi][wi] %= MOD;
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans += dp[i][j];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
