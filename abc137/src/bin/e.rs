use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        p:i64,
        abc:[(Usize1,Usize1,i64);m],
    }
    let mut g = vec![vec![]; n];
    let mut rev_g = vec![vec![]; n];
    for &(a, b, _) in &abc {
        g[a].push(b);
        rev_g[b].push(a);
    }

    let mut visited = vec![vec![false; 2]; n];
    visited[0][0] = true;
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(cur) = q.pop_front() {
        for &next in &g[cur] {
            if visited[next][0] {
                continue;
            }
            visited[next][0] = true;
            q.push_back(next);
        }
    }
    visited[n - 1][1] = true;
    q.push_back(n - 1);
    while let Some(cur) = q.pop_front() {
        for &next in &rev_g[cur] {
            if visited[next][1] {
                continue;
            }
            visited[next][1] = true;
            q.push_back(next);
        }
    }
    let mut edges = vec![];
    for &(a, b, c) in &abc {
        if visited[a][0] && visited[a][1] && visited[b][0] && visited[b][1] {
            edges.push((a, b, p - c));
        }
    }
    let bf = BellmanFord::new(n, edges, 0);
    if bf.has_neg_loop {
        println!("-1");
    } else {
        println!("{}", -bf.distance(n - 1).min(0));
    }
}

// 計算量はE×V
pub struct BellmanFord {
    distance: Vec<i64>,
    has_neg_loop: bool,
}

impl BellmanFord {
    // n:usize 頂点の数
    // edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: Vec<(usize, usize, i64)>, init: usize) -> Self {
        let mut distance = vec![1 << 60; n];
        distance[init] = 0;
        let mut has_neg_loop = false;

        for i in 0..n {
            for edge in &edges {
                let from = edge.0;
                let to = edge.1;
                let cost = edge.2;
                if distance[to] > distance[from] + cost {
                    distance[to] = distance[from] + cost;
                    if i == n - 1 {
                        has_neg_loop = true;
                        break;
                    }
                }
            }
        }
        Self {
            distance,
            has_neg_loop,
        }
    }

    pub fn distance(&self, target: usize) -> i64 {
        self.distance[target]
    }
}
