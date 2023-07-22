use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h:usize,
        w:usize,
        n:usize,
        ab:[(Usize1,Usize1);n],
    }
    let mut hole = vec![vec![false; w]; h];
    for (a, b) in ab {
        hole[a][b] = true;
    }
    let mut dp = vec![vec![0; w]; h];
    for i in 0..w {
        if !hole[0][i] {
            dp[0][i] = 1;
        }
    }
    for i in 0..h {
        if !hole[i][0] {
            dp[i][0] = 1;
        }
    }
    for i in 1..h {
        for j in 1..w {
            if hole[i][j] {
                dp[i][j] = 0;
            } else {
                dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
            }
        }
    }
    let mut ans = 0usize;
    for i in 0..h {
        for j in 0..w {
            ans += dp[i][j];
        }
    }
    println!("{}", ans);
}
