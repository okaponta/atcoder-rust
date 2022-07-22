use proconio::{input, marker::Chars};

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
       h:usize,
       w:usize,
       mut s:[Chars;h],
    }
    let mut dp = vec![vec![0; w]; h];
    let mut dpx = vec![vec![0; w]; h];
    let mut dpy = vec![vec![0; w]; h];
    let mut dpz = vec![vec![0; w]; h];
    dp[0][0] = 1;
    s[0][0] = '#';
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            if let Some(previ) = i.checked_sub(1) {
                dpx[i][j] = dpx[previ][j] + dp[previ][j];
            }
            if let Some(prevj) = j.checked_sub(1) {
                dpy[i][j] = dpy[i][prevj] + dp[i][prevj];
                if let Some(previ) = i.checked_sub(1) {
                    dpz[i][j] = dpz[previ][prevj] + dp[previ][prevj];
                }
            }
            dp[i][j] = dpx[i][j] + dpy[i][j] + dpz[i][j];
            dpx[i][j] %= MOD;
            dpy[i][j] %= MOD;
            dpz[i][j] %= MOD;
            dp[i][j] %= MOD;
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
