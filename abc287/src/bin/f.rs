use proconio::{fastout, input, marker::Usize1};

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    let ans = dfs(n, 0, 0, &edges);
    for i in 1..=n {
        println!("{}", (ans[i][0] + ans[i][1]) % MOD);
    }
}

fn dfs(n: usize, prev: usize, cur: usize, edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; 2]; 2];
    res[0][0] = 1;
    res[1][1] = 1;
    for &next in &edges[cur] {
        if next == prev {
            continue;
        }
        let child = dfs(n, cur, next, edges);
        let mut dp = vec![vec![0; 2]; res.len() + child.len() - 1];
        for i in 0..res.len() {
            for j in 0..child.len() {
                dp[i + j][0] += res[i][0] * (child[j][0] + child[j][1]);
                dp[i + j][1] += res[i][1] * child[j][0];
                dp[i + j][0] %= MOD;
                dp[i + j][1] %= MOD;
                if 0 < i + j {
                    dp[i + j - 1][1] += res[i][1] * child[j][1];
                    dp[i + j - 1][1] %= MOD;
                }
            }
        }
        res = dp;
    }
    res
}
