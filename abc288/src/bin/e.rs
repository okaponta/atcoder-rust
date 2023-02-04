use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        c:[usize;n],
        x:[Usize1;m],
    }
    let set = x.into_iter().collect::<HashSet<_>>();
    let mut cost = vec![vec![]; n];
    for i in 0..n {
        let mut min = a[i] + c[i];
        for j in 0..=i {
            min = min.min(a[i] + c[i - j]);
            cost[i].push(min);
        }
    }
    let mut dp = vec![0];
    for i in 0..n {
        let mut next = vec![1 << 60; i + 2];
        next[i + 1] = dp[i] + cost[i][i];
        for j in 0..i {
            next[j + 1] = next[j + 1].min(dp[j] + cost[i][j]);
        }
        if !set.contains(&i) {
            for j in 0..=i {
                next[j] = next[j].min(dp[j]);
            }
        }
        dp = next;
    }
    println!("{}", dp.iter().min().unwrap());
}
