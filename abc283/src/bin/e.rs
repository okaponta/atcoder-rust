use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        a:[[u8;w];h],
    }
    let mut e = vec![vec![vec![9; 2]; w + 2]; h + 2];
    for i in 0..h {
        for j in 0..w {
            e[i + 1][j + 1][0] = a[i][j];
            e[i + 1][j + 1][1] = 1 - a[i][j];
        }
    }
    let mut dp = vec![vec![vec![1 << 30; 2]; 2]; h + 2];
    dp[0][0][0] = 0;
    dp[0][1][0] = 1;
    for i in 1..=h {
        for j in 0..2 {
            for k in 0..2 {
                if is_ok(&e, i, w, k, j, 0) {
                    dp[i][0][j] = dp[i][0][j].min(dp[i - 1][j][k]);
                }
                if is_ok(&e, i, w, k, j, 1) {
                    dp[i][1][j] = dp[i][1][j].min(1 + dp[i - 1][j][k]);
                }
            }
        }
    }
    let mut ans = 1 << 30;
    for j in 0..2 {
        for k in 0..2 {
            ans = ans.min(dp[h][j][k]);
        }
    }
    println!("{}", if ans == 1 << 30 { -1 } else { ans });
}

fn is_ok(e: &Vec<Vec<Vec<u8>>>, i: usize, w: usize, prev: usize, cur: usize, next: usize) -> bool {
    for j in 1..=w {
        if e[i][j][cur] != e[i][j - 1][cur]
            && e[i][j][cur] != e[i][j + 1][cur]
            && e[i][j][cur] != e[i - 1][j][prev]
            && e[i][j][cur] != e[i + 1][j][next]
        {
            return false;
        }
    }
    true
}
