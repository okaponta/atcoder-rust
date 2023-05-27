use proconio::{input, marker::Chars};

fn main() {
    input! {
        x:usize,
        y:usize,
        z:usize,
        s:Chars,
    }
    let mut dp = vec![0, z];
    for i in 0..s.len() {
        let mut next = vec![1 << 60; 2];
        if s[i] == 'a' {
            // capsOFF
            next[0] = next[0].min(dp[0] + x);
            next[0] = next[0].min(dp[1] + z + x);
            // capsON
            next[1] = next[1].min(dp[0] + z + y);
            next[1] = next[1].min(dp[1] + y);
        } else {
            // capsOFF
            next[0] = next[0].min(dp[0] + y);
            next[0] = next[0].min(dp[1] + z + y);
            // capsON
            next[1] = next[1].min(dp[1] + x);
            next[1] = next[1].min(dp[0] + z + x);
        }
        dp = next;
    }
    println!("{}", dp[0].min(dp[1]));
}
