use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
        w:[usize;n],
        mut a:[usize;n],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let ord = (0..n)
        .into_iter()
        .sorted_by_key(|&i| w[i])
        .collect::<Vec<_>>();
    let mut c = vec![0; n];
    for &i in &ord {
        let mut dp = vec![0; w[i]];
        dp[0] = 1;
        for &next in &g[i] {
            if w[i] <= w[next] {
                continue;
            }
            for j in (0..w[i] - w[next]).rev() {
                dp[j + w[next]] = dp[j + w[next]].max(dp[j] + c[next]);
            }
        }
        c[i] = dp.into_iter().max().unwrap();
    }
    let mut ans = 0;
    for i in 0..n {
        ans += c[i] * a[i];
    }
    println!("{}", ans);
}
