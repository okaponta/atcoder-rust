use std::vec;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize, mut k:usize,
       a:[Usize1;n],
    }
    // dp[i][j] = 点jから2^i回遷移する先
    // 2^60まであれば今回は十分
    let mut dp = vec![a.clone(); 61];
    for i in 0..60 {
        for j in 0..n {
            dp[i + 1][j] = dp[i][dp[i][j]];
        }
    }
    let mut ans = 0;
    for i in 0..60 {
        if k & 1 == 1 {
            ans = dp[i][ans];
        }
        k >>= 1;
    }
    println!("{}", ans + 1);
}
