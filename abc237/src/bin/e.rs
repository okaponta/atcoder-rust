use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        h:[i64;n],
        uv:[(Usize1,Usize1);m],
    }
    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        if h[u] > h[v] {
            edge[u].push((v, 0));
            edge[v].push((u, h[u] - h[v]));
        } else {
            edge[u].push((v, h[v] - h[u]));
            edge[v].push((u, 0));
        }
    }
    let d = Dijkstra::new(n, edge, 0);
    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(h[0] - h[i] - d.potential[i]);
    }
    println!("{}", ans);
}

const INF: i64 = 1 << 60;

struct Dijkstra {
    potential: Vec<i64>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, i64)>>, init: usize) -> Self {
        let mut potential = vec![INF; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((Reverse(0), i));
            }
            heap.push((Reverse(INF), i));
        }
        while let Some((Reverse(d), target)) = heap.pop() {
            if potential[target] < d {
                continue;
            }
            potential[target] = d;
            for &(next, cost) in &edge[target] {
                if potential[next] > d + cost {
                    potential[next] = d + cost;
                    heap.push((Reverse(potential[next]), next));
                }
            }
        }
        Self { potential }
    }
}
