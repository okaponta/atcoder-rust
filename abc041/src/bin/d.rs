use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        xy:[(Usize1,Usize1);m],
    }
    let mut ng = vec![vec![]; n];
    for (x, y) in xy {
        ng[x].push(y);
    }
    let mut dp = vec![0usize; 1 << n];
    dp[0] = 1;
    for i in 0..1 << n {
        let mut ngs = HashSet::new();
        for j in 0..n {
            if i >> j & 1 == 1 {
                for &y in &ng[j] {
                    ngs.insert(y);
                }
            }
        }
        for j in 0..n {
            if i >> j & 1 == 1 || ngs.contains(&j) {
                continue;
            }
            dp[i | 1 << j] += dp[i];
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
