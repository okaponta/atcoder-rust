use proconio::input;

fn main() {
    input! {
        n:i32,
    }
    let mut dp = vec![1i64; 9];
    for _ in 1..n {
        let mut next = vec![0; 9];
        next[0] += dp[0] + dp[1];
        next[1] += dp[0] + dp[1] + dp[2];
        next[2] += dp[1] + dp[2] + dp[3];
        next[3] += dp[2] + dp[3] + dp[4];
        next[4] += dp[3] + dp[4] + dp[5];
        next[5] += dp[4] + dp[5] + dp[6];
        next[6] += dp[5] + dp[6] + dp[7];
        next[7] += dp[6] + dp[7] + dp[8];
        next[8] += dp[7] + dp[8];
        for i in 0..9 {
            next[i] %= 998244353;
        }
        dp = next;
    }
    let ans: i64 = dp.iter().sum();
    println!("{}", ans % 998244353);
}
