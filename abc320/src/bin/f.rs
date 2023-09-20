use proconio::input;

fn main() {
    input! {
        n:usize,
        h:usize,
        mut x:[usize;n],
        mut pf:[(usize,usize);n-1],
    }
    x.insert(0, 0);
    pf.insert(0, (0, 0));
    pf.push((0, 0));
    // dp[i][j][k]...往路でj、復路でkの燃料をもっているときに支払う金額の合計の最小値
    // 往路では使用後、復路では使用前
    let mut dp = vec![vec![vec![1 << 60; h + 1]; h + 1]; n + 1];
    for i in 0..=h {
        dp[0][h][i] = 0;
    }
    for i in 1..=n {
        let d = x[i] - x[i - 1];
        for j in d..=h {
            for k in 0..=h - d {
                // 往路使わない復路使わない
                dp[i][j - d][k + d] = dp[i][j - d][k + d].min(dp[i - 1][j][k]);
                // 往路使う
                dp[i][(j - d + pf[i].1).min(h)][k + d] =
                    dp[i][(j - d + pf[i].1).min(h)][k + d].min(dp[i - 1][j][k] + pf[i].0);

                let k2 = (k + pf[i].1).min(h);
                if d <= k2 {
                    dp[i][j - d][k] = dp[i][j - d][k].min(dp[i - 1][j][k2 - d] + pf[i].0);
                }
            }
        }
    }
    let mut ans = 1 << 60;
    for i in 0..=h {
        ans = ans.min(dp[n][i][i]);
    }
    if ans == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
