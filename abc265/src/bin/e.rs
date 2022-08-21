use std::collections::HashSet;

use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:i64,b:i64,c:i64,d:i64,e:i64,f:i64,
        xy:[(i64,i64);m],
    }
    let mut ng = HashSet::new();
    for (x, y) in xy {
        ng.insert((x, y));
    }
    // dp[i][j] = 1のワープをi回、2のワープをj回
    let mut dp = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    for t in 0..n {
        let mut next = vec![vec![0; n + 1]; n + 1];
        for i in 0..=t {
            for j in 0..=t - i {
                let k = t - i - j;
                let x = a * i as i64 + c * j as i64 + e * k as i64;
                let y = b * i as i64 + d * j as i64 + f * k as i64;
                if !ng.contains(&(x + a, y + b)) {
                    next[i + 1][j] += dp[i][j];
                    next[i + 1][j] %= MOD;
                }
                if !ng.contains(&(x + c, y + d)) {
                    next[i][j + 1] += dp[i][j];
                    next[i][j + 1] %= MOD;
                }
                if !ng.contains(&(x + e, y + f)) {
                    next[i][j] += dp[i][j];
                    next[i][j] %= MOD;
                }
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for i in 0..=n {
        for j in 0..=n - i {
            ans += dp[i][j];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
