use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
        c:[usize;n],
    }
    // dp[0] 0101010
    // dp[1] 0101101 (1回反転済)
    // dp[2] 1010101
    // dp[3] 1010110 (1回反転済)
    let mut dp = vec![1 << 60; 4];
    if s[0] == '0' {
        dp[0] = 0;
        dp[2] = c[0];
    } else {
        dp[0] = c[0];
        dp[2] = 0;
    }
    for i in 1..n {
        let mut next = vec![1 << 60; 4];
        if (s[i] == '0' && i % 2 == 0) || (s[i] == '1' && i % 2 == 1) {
            next[0] = next[0].min(dp[0]);
            next[1] = next[1].min(dp[1] + c[i]);
            next[1] = next[1].min(dp[0] + c[i]);
            next[2] = next[2].min(dp[2] + c[i]);
            next[3] = next[3].min(dp[3]);
            next[3] = next[3].min(dp[2]);
        } else {
            next[0] = next[0].min(dp[0] + c[i]);
            next[1] = next[1].min(dp[1]);
            next[1] = next[1].min(dp[0]);
            next[2] = next[2].min(dp[2]);
            next[3] = next[3].min(dp[3] + c[i]);
            next[3] = next[3].min(dp[2] + c[i]);
        }
        dp = next;
    }
    println!("{}", dp[1].min(dp[3]));
}
