use proconio::{marker::*, *};
use superslice::*;

fn main() {
    input! {
        n:usize,
        x:usize,
        vac:[(Usize1,usize,usize);n],
    }
    let mut dp = vec![vec![0; x + 1]; 3];
    for (v, a, c) in vac {
        for i in (0..=x).rev() {
            if i + c <= x {
                dp[v][i + c] = dp[v][i + c].max(dp[v][i] + a);
            }
        }
    }
    let mut l = 0;
    let mut u = 1 << 60;
    while 1 < u - l {
        let m = (l + u) / 2;
        let s: usize = (0..3).into_iter().map(|i| dp[i].lower_bound(&m)).sum();
        if s <= x {
            l = m;
        } else {
            u = m;
        }
    }
    println!("{}", l);
}
