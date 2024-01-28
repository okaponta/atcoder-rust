use proconio::input;

fn main() {
    input! {
        mut n:usize,
        a:[i64;2*n],
    }
    n *= 2;
    let mut dp = vec![vec![1 << 60; n]; n];
    for i in 1..n {
        dp[i - 1][i] = (a[i - 1] - a[i]).abs();
    }
    for l in (0..n).rev() {
        for r in (l + 1..n).step_by(2) {
            dp[l][r] = dp[l][r].min(dp[l + 1][r - 1] + (a[l] - a[r]).abs());
            for i in (l + 1..r).step_by(2) {
                dp[l][r] = dp[l][r].min(dp[l][i] + dp[i + 1][r]);
            }
        }
    }
    println!("{}", dp[0][n - 1]);
}
