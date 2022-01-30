use itertools::iproduct;
use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,m:usize,
    }
    let mut dp = vec![vec![vec![0; m + 2]; m + 2]; m + 2];
    dp[m + 1][m + 1][m + 1] = 1;
    for _ in 0..n {
        let mut next = vec![vec![vec![0; m + 2]; m + 2]; m + 2];
        for (a, b, c) in iproduct!(1..=m + 1, 1..=m + 1, 1..=m + 1) {
            let d = dp[a][b][c];
            for x in 1..=m {
                if x <= a {
                    next[x][b][c] = (next[x][b][c] + d) % MOD;
                } else if x <= b {
                    next[a][x][c] = (next[a][x][c] + d) % MOD;
                } else if x <= c {
                    next[a][b][x] = (next[a][b][x] + d) % MOD;
                }
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for i in 0..=m {
        for j in i + 1..=m {
            for k in j + 1..=m {
                ans = (ans + dp[i][j][k]) % MOD
            }
        }
    }
    println!("{}", ans);
}
