use proconio::input;

fn main() {
    input! {
        n:usize,
        t:usize,
        mut ab:[(usize,i64);n],
    }
    ab.sort();
    let mut dp = vec![-1 << 60; t + 1];
    dp[0] = 0;
    for (a, b) in ab {
        for i in (0..t).rev() {
            dp[(i + a).min(t)] = dp[(i + a).min(t)].max(dp[i] + b);
        }
    }
    let mut ans = 0;
    for i in 0..=t {
        ans = ans.max(dp[i]);
    }
    println!("{}", ans);
}
