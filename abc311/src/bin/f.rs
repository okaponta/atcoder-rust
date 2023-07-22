use itertools::iproduct;
use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        mut s:[Chars;n],
    }
    for (i, j) in iproduct!(0..n, 0..m) {
        if s[i][j] == '#' {
            if i != n - 1 {
                s[i + 1][j] = '#';
                if j != m - 1 {
                    s[i + 1][j + 1] = '#';
                }
            }
        }
    }
    let mut dp = vec![0; m + 1];
    dp[m] = 1;
    for c in 1..m + n {
        for j in (0..m).rev() {
            dp[j] += dp[j + 1];
            dp[j] %= MOD;
        }
        for j in 0..m {
            let i = j + c;
            if i < m {
                dp[j] = 0;
            } else if m + n <= i || s[i - m][j] == '#' {
                dp[j + 1] = 0;
            }
        }
    }
    println!("{}", (dp[0] + dp[1]) % MOD);
}
