use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let coins = vec![0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1];
    let mut dp = vec![0usize, 10];
    for c in s {
        let c = c.to_digit(10).unwrap() as usize;
        let a = (dp[0] + coins[c]).min(dp[1] + coins[10 - c]);
        let mut b = dp[0] + coins[c + 1];
        if c != 0 {
            b = b.min(dp[1] + coins[10 - c - 1]);
        }
        dp = vec![a, b];
    }
    println!("{}", dp[0]);
}
