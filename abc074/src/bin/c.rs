use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
        d:usize,
        e:usize,
        f:usize,
    }
    let mut dp = vec![-1i64; f + 3010];
    dp[100 * a] = 0;
    dp[100 * b] = 0;
    let max = (100 + e, e);
    for i in 0..f {
        if dp[i] < 0 {
            continue;
        }
        dp[i + 100 * a] = dp[i + 100 * a].max(dp[i]);
        dp[i + 100 * b] = dp[i + 100 * b].max(dp[i]);
        if max.0 * (dp[i] as usize + c) <= (i + c) * max.1 {
            dp[i + c] = dp[i + c].max(dp[i] + c as i64);
        }
        if max.0 * (dp[i] as usize + d) <= (i + d) * max.1 {
            dp[i + d] = dp[i + d].max(dp[i] + d as i64);
        }
    }

    let mut ans = (100 * a as i64, 0i64);
    for i in 0..=f {
        if i as i64 * ans.1 < ans.0 * dp[i] {
            ans = (i as i64, dp[i]);
        }
    }
    println!("{} {}", ans.0, ans.1);
}
