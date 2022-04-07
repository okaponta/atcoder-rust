use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       n:usize,
       mut xc:[(i64,usize);n],
    }
    let cmap = compress(&xc);
    let mut v = vec![(1 << 60, -1 << 60); cmap.len()];
    for (x, c) in xc {
        let (mut min, mut max) = v[cmap[&c]];
        min = min.min(x);
        max = max.max(x);
        v[cmap[&c]] = (min, max);
    }
    // dp[i]=(sum,prev) iは0or1, prevは直前の数
    let mut dp = vec![(0, 0); 2];
    for (min, max) in v {
        let mut next = vec![(1 << 60, 0); 2];
        // min -> max が0
        for (sum, prev) in &dp {
            let n_zero = sum + (prev - min).abs() + (max - min).abs();
            if n_zero < next[0].0 {
                next[0] = (n_zero, max);
            }
            let n_one = sum + (prev - max).abs() + (max - min).abs();
            if n_one < next[1].0 {
                next[1] = (n_one, min);
            }
        }
        dp = next;
    }
    let ans = (dp[0].0 + dp[0].1.abs()).min(dp[1].0 + dp[1].1.abs());
    println!("{}", ans);
}

fn compress(source: &[(i64, usize)]) -> HashMap<usize, usize> {
    let set: HashSet<&usize> = source.iter().map(|e| &e.1).collect();
    let mut result: HashMap<usize, usize> = HashMap::new();
    for (i, x) in set.into_iter().sorted().enumerate() {
        result.insert(*x, i);
    }
    result
}
