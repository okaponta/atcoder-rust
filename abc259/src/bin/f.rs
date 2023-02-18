use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        mut d:[usize;n],
        mut uvw:[(Usize1,Usize1,i64);n-1],
    }
    let mut g = vec![vec![]; n];
    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut dp = vec![(-1 << 60, 0); n];
    dfs(0, 0, &mut dp, &g, &d);
    println!("{}", dp[0].1);
}

fn dfs(
    cur: usize,
    prev: usize,
    dp: &mut Vec<(i64, i64)>,
    g: &Vec<Vec<(usize, i64)>>,
    d: &Vec<usize>,
) {
    let mut sum = 0;
    let mut delta = vec![0];
    for &(next, cost) in &g[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, dp, g, d);
        sum += dp[next].1;
        delta.push((cost + dp[next].0 - dp[next].1).max(0));
    }
    delta.sort_by_key(|i| -i);
    let mut tmp = 0;
    for i in 0..d[cur] {
        if i == d[cur] - 1 {
            dp[cur].0 = sum + tmp;
        }
        tmp += delta[i];
    }
    dp[cur].1 = sum + tmp;
}
