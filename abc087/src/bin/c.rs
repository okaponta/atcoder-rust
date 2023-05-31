use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[[usize;n];2],
    }
    let mut dp = vec![vec![0; n]; 2];
    dp[0][0] = a[0][0];
    for i in 1..n {
        dp[0][i] = dp[0][i - 1] + a[0][i];
    }
    dp[1][0] = a[0][0] + a[1][0];
    for i in 1..n {
        dp[1][i] = dp[1][i - 1].max(dp[0][i]) + a[1][i];
    }
    println!("{}", dp[1][n - 1]);
}
