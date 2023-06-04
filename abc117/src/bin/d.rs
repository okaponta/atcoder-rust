use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut dp = vec![vec![-1 << 60; 2]; 41];
    dp[40][0] = 0;
    for i in (0..40).rev() {
        let mut count = 0i64;
        for j in 0..n {
            if a[j] >> i & 1 == 1 {
                count += 1;
            }
        }
        let zero = (1 << i) * count;
        let one = (1 << i) * (n as i64 - count);
        if k >> i & 1 == 1 {
            dp[i][0] = dp[i][0].max(dp[i + 1][0] + one);
            dp[i][1] = dp[i][1].max(dp[i + 1][0] + zero);
            dp[i][1] = dp[i][1].max(dp[i + 1][1] + zero.max(one));
        } else {
            dp[i][0] = dp[i][0].max(dp[i + 1][0] + zero);
            dp[i][1] = dp[i][1].max(dp[i + 1][1] + zero.max(one));
        }
    }
    println!("{}", dp[0][0].max(dp[0][1]))
}
