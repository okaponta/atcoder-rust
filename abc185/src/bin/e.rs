use itertools::iproduct;
use proconio::input;

const INF: usize = 1 >> 60;

fn main() {
    input! {
       n:usize,m:usize,
       a:[usize;n],
       b:[usize;m],
    }
    let mut dp = vec![vec![INF; m + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = i
    }
    for j in 0..=m {
        dp[0][j] = j
    }
    for (i, j) in iproduct!(1..=n, 1..=m) {
        let x = dp[i - 1][j] + 1;
        let y = dp[i][j - 1] + 1;
        let z = dp[i - 1][j - 1] + if a[i - 1] == b[j - 1] { 0 } else { 1 };
        dp[i][j] = x.min(y).min(z);
    }
    println!("{}", dp[n][m]);
}
