use std::collections::VecDeque;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
        k:usize,
        c:[Usize1;k],
    }
    let mut uf = UnionFind::new(n);
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        uf.union(a, b);
        edges[a].push(b);
        edges[b].push(a);
    }
    if !c.iter().all(|&ci| uf.find(ci) == uf.find(c[0])) {
        println!("-1");
        return;
    }
    let mut dist = vec![vec![0; k]; k];
    for i in 0..k {
        let d = bfs(c[i], n, &edges);
        for j in i..k {
            dist[i][j] = d[c[j]];
            dist[j][i] = d[c[j]];
        }
    }
    let mut dp = vec![vec![1 << 60; k]; 1 << k];
    for i in 0..k {
        dp[1 << i][i] = 1;
    }
    for s in 1..1 << k {
        for i in 0..k {
            // 次がi
            if s >> i & 1 != 1 {
                //前がj
                for j in 0..k {
                    if i != j {
                        dp[s | 1 << i][i] = dp[s | 1 << i][i].min(dp[s][j] + dist[i][j]);
                    }
                }
            }
        }
    }
    let ans = dp[(1 << k) - 1].iter().min().unwrap();
    println!("{}", ans);
}

fn bfs(init: usize, n: usize, edges: &Vec<Vec<usize>>) -> Vec<i64> {
    let mut d = vec![-1; n];
    let mut q = VecDeque::new();
    q.push_back((init, 0));
    while let Some((cur, dist)) = q.pop_front() {
        if d[cur] != -1 {
            continue;
        }
        d[cur] = dist;
        for &next in &edges[cur] {
            if d[next] == -1 {
                q.push_back((next, dist + 1));
            }
        }
    }
    d
}
