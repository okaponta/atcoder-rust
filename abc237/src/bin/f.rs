use itertools::iproduct;
use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,m:usize,
    }
    let mut dp = vec![vec![vec![0; m + 1]; m + 1]; m + 1];
    dp[0][0][0] = 1;
    for _ in 0..n {
        let mut next = vec![vec![vec![0; m + 1]; m + 1]; m + 1];
        for (a, b, c) in iproduct!(0..=m, 0..=m, 0..=m) {
            let d = dp[a][b][c];
            for x in 1..=m {
                if x <= a || a == 0 {
                    next[x][b][c] = (next[x][b][c] + d) % MOD;
                } else if x <= b || b == 0 {
                    next[a][x][c] = (next[a][x][c] + d) % MOD;
                } else if x <= c || c == 0 {
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
