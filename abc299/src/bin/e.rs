use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
        k:usize,
        pd:[(Usize1,usize);k],
    }
    let mut dist = vec![vec![vec![]; n + 1]; n];
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    for i in 0..n {
        let d = Dijkstra::new(n, &g, i);
        for j in 0..n {
            dist[i][d.distance[j]].push(j);
        }
    }
    let mut cond = vec![vec![]; k];
    let mut ans = vec![1; n];
    for (i, &(p, d)) in pd.iter().enumerate() {
        for &c in &dist[p][d] {
            if ans[c] != 0 {
                cond[i].push(c);
            }
        }
        for i in 0..d {
            for &c in &dist[p][i] {
                ans[c] = 0;
            }
        }
    }
    for v in cond {
        let mut count = 0;
        for i in v {
            if ans[i] != 0 {
                count += 1;
            }
        }
        if count == 0 {
            println!("No");
            return;
        }
    }
    println!("Yes");
    println!("{}", ans.iter().join(""));
}

pub struct Dijkstra {
    distance: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<usize>>, init: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut distance = vec![INF; n];
        let mut heap = std::collections::BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((std::cmp::Reverse(0), i));
            }
            heap.push((std::cmp::Reverse(INF), i));
        }
        while let Some((std::cmp::Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &next in &edges[target] {
                if distance[next] > d + 1 {
                    distance[next] = d + 1;
                    heap.push((std::cmp::Reverse(distance[next]), next));
                }
            }
        }
        Self { distance }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }
}
