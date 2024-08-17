use proconio::*;

fn main() {
    input! {
        n:usize,
        x:usize,
        y:usize,
        ab:[(usize,usize);n],
    }
    let mut dp = vec![vec![1 << 60; x + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in (0..=i).rev() {
            for k in 0..x {
                if x < k + a {
                    break;
                }
                dp[j + 1][k + a] = dp[j + 1][k + a].min(dp[j][k] + b);
            }
        }
    }
    for i in (0..=n).rev() {
        for j in 0..=x {
            if dp[i][j] <= y {
                println!("{}", (i + 1).min(n));
                return;
            }
        }
    }
}
