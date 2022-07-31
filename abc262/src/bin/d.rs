use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = n;
    for ni in 2..=n {
        // dp[i][j] = 選んだのがiこであまりがj
        let mut dp = vec![vec![0; ni]; ni];
        dp[0][0] = 1;
        for k in 0..n {
            let num = a[k] % ni;
            for i in (0..ni).rev() {
                if i == ni - 1 {
                    ans += dp[i][(ni - num) % ni];
                    ans %= MOD;
                } else {
                    for j in 0..ni {
                        dp[i + 1][(j + num) % ni] += dp[i][j];
                        dp[i + 1][(j + num) % ni] %= MOD;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
