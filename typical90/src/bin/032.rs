use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        a:[[usize;n];n],
        m:usize,
        xy:[(Usize1,Usize1);m],
    }
    let mut ng = HashSet::new();
    for (x, y) in xy {
        ng.insert((x, y));
        ng.insert((y, x));
    }
    let mut dp = vec![vec![1 << 60; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = a[i][0];
    }
    for i in 1usize..1 << n {
        let nth = i.count_ones() as usize;
        for j in 0..n {
            if i >> j & 1 == 0 {
                // 未訪問
                continue;
            }
            for k in 0..n {
                if i >> k & 1 == 1 {
                    // 訪問済み
                    continue;
                }
                if ng.contains(&(j, k)) {
                    continue;
                }
                dp[i | 1 << k][k] = dp[i | 1 << k][k].min(dp[i][j] + a[k][nth]);
            }
        }
    }
    let mut ans = 1 << 60;
    for i in 0..n {
        ans = ans.min(dp[(1 << n) - 1][i]);
    }
    if ans == 1 << 60 {
        println!("-1");
        return;
    }
    println!("{}", ans);
}
