use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w:usize,
        r:[usize;h],
        c:[usize;w],
        ac:[Chars;h],
    }
    let mut a = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if ac[i][j] == '1' {
                a[i][j] = 1;
            }
        }
    }
    let black = solve(h, w, &r, &c, &a, 1);
    let white = solve(h, w, &r, &c, &a, 0);
    println!("{}", black.min(white));
}

fn solve(
    h: usize,
    w: usize,
    r: &Vec<usize>,
    c: &Vec<usize>,
    a: &Vec<Vec<u8>>,
    target: u8,
) -> usize {
    // dp[i][j][k] i行j列目で現在いる
    // k=0 行、列どれも反転してない, k=1 行が反転している, k=2 列が反転している, k=3 どっちも反転している
    let mut dp = vec![vec![vec![1 << 60; 4]; w]; h];
    if a[0][0] == target {
        dp[0][0][0] = 0;
        dp[0][0][3] = r[0] + c[0];
    } else {
        dp[0][0][1] = r[0];
        dp[0][0][2] = c[0];
    }
    for i in 0..h {
        for j in 0..w {
            if i != h - 1 {
                if a[i + 1][j] == target {
                    // 反転不要
                    dp[i + 1][j][0] = dp[i + 1][j][0].min(dp[i][j][0]).min(dp[i][j][1]);
                    dp[i + 1][j][3] = dp[i + 1][j][3]
                        .min(dp[i][j][3] + r[i + 1])
                        .min(dp[i][j][2] + r[i + 1]);
                } else {
                    dp[i + 1][j][1] = dp[i + 1][j][1]
                        .min(dp[i][j][0] + r[i + 1])
                        .min(dp[i][j][1] + r[i + 1]);
                    dp[i + 1][j][2] = dp[i + 1][j][2].min(dp[i][j][2]).min(dp[i][j][3]);
                }
            }
            if j != w - 1 {
                if a[i][j + 1] == target {
                    // 反転不要
                    dp[i][j + 1][0] = dp[i][j + 1][0].min(dp[i][j][0]).min(dp[i][j][2]);
                    dp[i][j + 1][3] = dp[i][j + 1][3]
                        .min(dp[i][j][3] + c[j + 1])
                        .min(dp[i][j][1] + c[j + 1]);
                } else {
                    dp[i][j + 1][1] = dp[i][j + 1][1].min(dp[i][j][1]).min(dp[i][j][3]);
                    dp[i][j + 1][2] = dp[i][j + 1][2]
                        .min(dp[i][j][2] + c[j + 1])
                        .min(dp[i][j][0] + c[j + 1]);
                }
            }
        }
    }
    *dp[h - 1][w - 1].iter().min().unwrap()
}
