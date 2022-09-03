use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[i64;n]
    }
    let mut dp = vec![-1 << 60; m + 1];
    dp[0] = 0;
    for i in 0..n {
        for j in (0..m).rev() {
            dp[j + 1] = dp[j + 1].max(dp[j] + a[i] * (j as i64 + 1));
        }
    }
    println!("{}", dp[m]);
}
