use proconio::input;

fn main() {
    input! {
       n:usize,
       mut a:[usize;n],
       mut b:[usize;n],
    }
    a.insert(0, 0);
    b.insert(0, 0);
    // 頂点0の色は色0
    // dp[i][j][k] i番目まで考えたときの答え(iは色はj、1の色はk)
    let mut dp = vec![vec![vec![1 << 60; 2]; 2]; n + 1];
    dp[1][0][0] = a[1];
    dp[1][1][1] = 0;
    for i in 2..=n {
        for k in 0..2 {
            dp[i][0][k] = (dp[i - 1][0][k] + a[i] + b[i - 1]).min(dp[i - 1][1][k] + a[i]);
            dp[i][1][k] = dp[i - 1][0][k].min(dp[i - 1][1][k] + b[i - 1]);
        }
    }
    let ans = (dp[n][0][0].min(dp[n][1][1]) + b[n]).min(dp[n][0][1].min(dp[n][1][0]));
    println!("{}", ans);
}
