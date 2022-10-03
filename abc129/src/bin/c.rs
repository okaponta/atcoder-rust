use std::collections::HashSet;

use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
    }
    let set = a.into_iter().collect::<HashSet<usize>>();
    let mut dp = vec![0; n + 2];
    dp[0] = 1;
    for i in 0..n {
        for j in 1..3 {
            if !set.contains(&(i + j)) {
                dp[i + j] += dp[i];
                dp[i + j] %= MOD;
            }
        }
    }
    println!("{}", dp[n]);
}
