use proconio::{input, marker::Chars};

const MOD: usize = 1_000_000_007;

fn main() {
    input! {s:Chars}
    let mut dp = vec![0, 0, 0, 1];
    for c in s.into_iter().rev() {
        let mut next = dp.clone();
        if c == '?' {
            next[0] = (dp[0] * 3 + dp[1]) % MOD;
            next[1] = (dp[1] * 3 + dp[2]) % MOD;
            next[2] = (dp[2] * 3 + dp[3]) % MOD;
            next[3] = (dp[3] * 3) % MOD;
        } else if c == 'A' {
            next[0] = (next[0] + dp[1]) % MOD;
        } else if c == 'B' {
            next[1] = (next[1] + dp[2]) % MOD;
        } else {
            next[2] = (next[2] + dp[3]) % MOD;
        }
        dp = next;
    }
    println!("{}", dp[0]);
}
