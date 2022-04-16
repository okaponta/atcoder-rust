use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
    }
    let k = k - n;
    let mut dp = vec![vec![0; k + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 1;
    }
    for i in 0..n {
        for j in 1..=k {
            dp[i + 1][j] = dp[i + 1][j - 1] + dp[i][j];
            if j >= m {
                dp[i + 1][j] += MOD - dp[i][j - m];
            }
            dp[i + 1][j] %= MOD;
        }
    }
    println!("{}", dp[n].iter().fold(0, |a, x| (a + x) % MOD));
}
