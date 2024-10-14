#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:f64,
        t:f64,
        abcd:[(f64,f64,f64,f64);n],
    }
    let mut p = vec![];
    for (a, b, c, d) in abcd {
        p.push((a, b));
        p.push((c, d));
    }
    let m = 2 * n;
    let mut dp = vec![vec![1e7; m]; 1 << m];
    for i in 0..m {
        dp[1 << i][i] = time(p[i].0, p[i].1, s);
    }
    for i in 0usize..1 << m {
        let count = i.count_ones();
        for j in 0..m {
            if count % 2 == 0 {
                // 次
                for k in 0..m {
                    if i >> k & 1 == 1 {
                        //訪問済み
                        continue;
                    }
                    dp[i | 1 << k][k] =
                        dp[i | 1 << k][k].min(dp[i][j] + time(p[k].0 - p[j].0, p[k].1 - p[j].1, s))
                }
            } else {
                let next = pair(j);
                dp[i | 1 << next][next] = dp[i | 1 << next][next]
                    .min(dp[i][j] + time(p[next].0 - p[j].0, p[next].1 - p[j].1, t));
            }
        }
    }
    let mut ans: f64 = 1e9;
    for i in 0..m {
        ans = ans.min(dp[(1 << m) - 1][i]);
    }
    println!("{}", ans);
}

fn time(x: f64, y: f64, v: f64) -> f64 {
    (x * x + y * y).sqrt() / v
}

fn pair(i: usize) -> usize {
    if i % 2 == 0 {
        i + 1
    } else {
        i - 1
    }
}
