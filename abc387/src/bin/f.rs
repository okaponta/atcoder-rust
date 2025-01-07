use petgraph::unionfind::UnionFind;
use proconio::{marker::*, *};

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[Usize1;n],
    }
    let mut uf = UnionFind::new(n);
    let mut g = vec![vec![]; n];
    let mut cycle = vec![];
    for i in 0..n {
        g[a[i]].push(i);
        if !uf.union(i, a[i]) {
            cycle.push(i);
        }
    }
    let mut map = (0..n).into_iter().collect::<Vec<_>>();
    for &i in &cycle {
        let mut tmp = a[i];
        while tmp != i {
            map[tmp] = i;
            tmp = a[tmp];
        }
    }
    let mut h = vec![vec![]; n];
    for i in 0..n {
        if map[a[i]] != map[i] {
            h[map[a[i]]].push(map[i]);
        }
    }
    let mut dp = vec![vec![1; m]; n];
    let mut dps = vec![vec![1; m]; n];
    let mut ans = 1;
    for &i in &cycle {
        dfs(i, &h, &mut dp, &mut dps, m);
        ans *= dps[i][m - 1];
        ans %= MOD;
    }
    println!("{}", ans);
}

fn dfs(
    cur: usize,
    g: &Vec<Vec<usize>>,
    dp: &mut Vec<Vec<usize>>,
    dps: &mut Vec<Vec<usize>>,
    m: usize,
) {
    for &next in &g[cur] {
        dfs(next, g, dp, dps, m);
    }
    for i in 1..m {
        for &next in &g[cur] {
            dp[cur][i] *= dps[next][i];
            dp[cur][i] %= MOD;
        }
    }
    for i in 1..m {
        dps[cur][i] = (dps[cur][i - 1] + dp[cur][i]) % MOD;
    }
}
