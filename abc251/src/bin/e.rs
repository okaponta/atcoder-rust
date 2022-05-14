use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    // dp[i][j] iまでに配った最小の費用。j=1のときn番目に配給済み
    let mut dp = vec![vec![1 << 60; 2]; n];
    dp[1][0] = a[0];
    dp[0][1] = a[n - 1];
    dp[1][1] = a[0] + a[n - 1];
    for i in 1..n - 1 {
        dp[i + 1][0] = (dp[i][0].min(dp[i - 1][0])) + a[i];
        dp[i + 1][1] = (dp[i][1].min(dp[i - 1][1])) + a[i];
    }
    let ans = dp[n - 1][0].min(dp[n - 1][1]).min(dp[n - 2][1]);
    println!("{}", ans);
}
