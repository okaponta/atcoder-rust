use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        l:usize,
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        dp[i + 1] += dp[i];
        dp[i + 1] %= MOD;
        if i + l <= n {
            dp[i + l] += dp[i];
            dp[i + l] %= MOD;
        }
    }
    println!("{}", dp[n]);
}
