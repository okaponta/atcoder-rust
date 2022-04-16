use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
    }
    // kを0許可に
    let k = k - n;
    // dp[i][j] i番目までで和がjの組み合わせ
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=k {
            for mm in 0..m {
                if j + mm <= k {
                    dp[i + 1][j + mm] = (dp[i + 1][j + mm] + dp[i][j]) % MOD;
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..=k {
        ans = (ans + dp[n][i]) % MOD;
    }
    println!("{}", ans);
}
