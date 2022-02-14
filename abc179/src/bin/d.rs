use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
       n:usize,k:usize,
       lr:[(usize,usize);k],
    }
    // 境界値考慮が面倒なので2倍の長さを確保
    let mut dp = vec![0; 2 * n + 1];
    dp[0] = 1;
    dp[1] = -1;
    for i in 0..n {
        for (l, r) in &lr {
            dp[i + l] += dp[i];
            dp[i + l] %= MOD;
            dp[i + r + 1] = (dp[i + r + 1] + MOD - dp[i]) % MOD;
        }
        dp[i + 1] += dp[i];
        dp[i + 1] %= MOD;
    }
    println!("{}", dp[n - 1]);
}
