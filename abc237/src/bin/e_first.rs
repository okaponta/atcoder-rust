use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,m:usize,
        h:[i64;n],
        uv:[(Usize1,Usize1);m],
    }
    let mut edge = vec![vec![]; n];
    for (u, v) in uv {
        edge[u].push(v);
        edge[v].push(u);
    }
    let d = Dijkstra::new(n, edge, h);
    println!("{}", d.fun.iter().max().unwrap());
}

const INF: i64 = 1 << 60;

struct Dijkstra {
    fun: Vec<i64>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<usize>> edge[i] = [2, 3, (頂点への道)]
    // height: Vec<i64> height[i] = 標高、これをもとに楽しさを計算する
    pub fn new(n: usize, edge: Vec<Vec<usize>>, height: Vec<i64>) -> Self {
        let mut fun = vec![-INF; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == 0 {
                heap.push((0, i));
            }
            heap.push((-INF, i));
        }
        while let Some((d, target)) = heap.pop() {
            if fun[target] > d {
                continue;
            }
            fun[target] = d;
            for &next in &edge[target] {
                let cost = if height[target] > height[next] {
                    height[target] - height[next]
                } else {
                    (height[target] - height[next]) * 2
                };
                if fun[next] < d + cost {
                    fun[next] = d + cost;
                    heap.push((fun[next], next));
                }
            }
        }
        Self { fun }
    }
}
