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
    let mut dp = vec![vec![1 << 60; h + 1]; h + 1];
    for i in 0..=h {
        dp[h][i] = 0;
    }
    for i in 1..=n {
        let mut next = vec![vec![1 << 60; h + 1]; h + 1];
        let d = x[i] - x[i - 1];
        for j in d..=h {
            for k in 0..=h - d {
                // 往路使わない復路使わない
                next[j - d][k + d] = next[j - d][k + d].min(dp[j][k]);
                // 往路使う
                next[(j - d + pf[i].1).min(h)][k + d] =
                    next[(j - d + pf[i].1).min(h)][k + d].min(dp[j][k] + pf[i].0);
                // 復路使う
                let k2 = (k + pf[i].1).min(h);
                if d <= k2 {
                    next[j - d][k] = next[j - d][k].min(dp[j][k2 - d] + pf[i].0);
                }
            }
        }
        dp = next;
    }
    let mut ans = 1 << 60;
    for i in 0..=h {
        ans = ans.min(dp[i][i]);
    }
    if ans == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
