use proconio::{input, marker::Chars};

fn main() {
    input! {
       h:usize,w:usize,
       a:[Chars;h],
    }
    if h == 1 && w == 1 {
        println!("Draw");
        return;
    }
    let mut plus = vec![vec![false; w]; h];
    let mut dp = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '+' {
                plus[i][j] = true;
            }
        }
    }
    for i in (0..h).rev() {
        for j in (0..w).rev() {
            let is_a = (i + j) % 2 == 1;
            if i == h - 1 && j == w - 1 {
                if is_a {
                    dp[i][j] = if plus[i][j] { 1 } else { -1 };
                } else {
                    dp[i][j] = if plus[i][j] { -1 } else { 1 };
                }
                continue;
            }
            if is_a {
                let mut a = 5000;
                let mut b = 5000;
                if i != h - 1 {
                    a = dp[i + 1][j];
                }
                if j != w - 1 {
                    b = dp[i][j + 1];
                }
                dp[i][j] = a.min(b) + if plus[i][j] { 1 } else { -1 };
            } else {
                let mut a = -5000;
                let mut b = -5000;
                if i != h - 1 {
                    a = dp[i + 1][j];
                }
                if j != w - 1 {
                    b = dp[i][j + 1];
                }
                dp[i][j] = a.max(b) + if plus[i][j] { -1 } else { 1 };
            }
        }
    }
    let ans = if h == 1 {
        dp[0][1]
    } else if w == 1 {
        dp[1][0]
    } else {
        dp[1][0].max(dp[0][1])
    };
    if ans < 0 {
        println!("Aoki");
    } else if ans == 0 {
        println!("Draw");
    } else {
        println!("Takahashi");
    }
}
