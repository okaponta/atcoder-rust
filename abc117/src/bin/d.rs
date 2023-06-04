use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut dp = vec![vec![-1 << 62; 2]; 41];
    dp[0][0] = 0;
    for i in 0..40 {
        let mut count = 0i64;
        for j in 0..n {
            if a[j] >> (39 - i) & 1 == 1 {
                count += 1;
            }
        }
        let zero = (1 << (39 - i)) * count;
        let one = (1 << (39 - i)) * (n as i64 - count);
        // 0
        dp[i + 1][0] = dp[i + 1][0].max(dp[i][0] + zero);
        dp[i + 1][1] = dp[i + 1][1].max(dp[i][1] + zero);
        if k >> (39 - i) & 1 == 1 {
            dp[i + 1][1] = dp[i + 1][1].max(dp[i][0] + zero);
        }
        // 1
        dp[i + 1][1] = dp[i + 1][1].max(dp[i][1] + one);
        if k >> (39 - i) & 1 == 1 {
            dp[i + 1][0] = dp[i + 1][0].max(dp[i][0] + one);
        }
    }
    println!("{}", dp[40][0].max(dp[40][1]))
}
