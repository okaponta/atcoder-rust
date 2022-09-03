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
        let mut next = dp.clone();
        for j in 0..m {
            next[j + 1] = next[j + 1].max(dp[j] + a[i] * (j as i64 + 1));
        }
        dp = next;
    }
    println!("{}", dp[m]);
}
