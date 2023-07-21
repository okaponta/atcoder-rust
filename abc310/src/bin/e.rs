use proconio::{input, marker::Chars};

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut dp = vec![0, 0];
    let mut ans = 0usize;
    for i in 0..n {
        if s[i] == '0' {
            dp[1] += dp[0];
            dp[0] = 1;
        } else {
            dp.swap(0, 1);
            dp[1] += 1;
        }
        ans += dp[1];
    }
    println!("{}", ans);
}
