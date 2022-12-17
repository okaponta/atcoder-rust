use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut edges = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
        uf.union(u, v);
    }
    let mut color = vec![0; n];
    for i in 0..n {
        if color[i] == 0 {
            if !dfs(i, 1, &mut color, &edges) {
                println!("0");
                return;
            }
        }
    }
    let mut map = HashMap::new();
    for i in 0..n {
        (*map.entry(uf.find(i)).or_insert(vec![])).push(i);
    }
    let mut ans = 0;
    for (_, v) in map {
        let mut white = 0;
        let mut black = 0;
        for i in v {
            if color[i] == 1 {
                white += 1;
            } else {
                black += 1;
            }
        }
        ans += 2 * white * black + (white + black) * (n - white - black);
    }
    println!("{}", ans / 2 - m);
}

fn dfs(cur: usize, col: i32, color: &mut Vec<i32>, edge: &Vec<Vec<usize>>) -> bool {
    color[cur] = col;
    for &next in &edge[cur] {
        if color[next] == col {
            return false;
        }
        if color[next] == 0 {
            if !dfs(next, -col, color, edge) {
                return false;
            }
        }
    }
    true
}
