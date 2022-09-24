use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;k],
    }
    let mut dp = vec![vec![-1; 2]; 2 * n];
    dp[0][0] = 0;
    dp[0][1] = 0;
    for i in 0..n {
        for j in 0..k {
            if dp[i][0] != -1 {
                dp[i + a[j]][1] = dp[i + a[j]][1].max(dp[i][0] + a[j] as i64);
            }
            if dp[i][1] != -1 {
                if dp[i + a[j]][0] != -1 {
                    dp[i + a[j]][0] = dp[i + a[j]][0].min(dp[i][1]);
                } else {
                    dp[i + a[j]][0] = dp[i][1];
                }
            }
        }
    }
    println!("{}", dp[n][0].max(dp[n][1]));
}
