use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        abc:[(Usize1,Usize1,usize);m],
    }
    let mut edges = vec![vec![]; n];
    let mut map = HashMap::new();
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        edges[a].push((b, c));
        edges[b].push((a, c));
        map.insert((a, b), i + 1);
    }
    let d = Dijkstra::new(n, edges, 0);
    let mut ans = vec![];
    for (a, b) in d.edges {
        let idx = map.get(&(a.min(b), a.max(b))).unwrap();
        ans.push(idx);
    }
    println!("{}", ans.iter().join(" "));
}

const INF: usize = 1 << 60;

// 計算量は(E+V)logV
struct Dijkstra {
    edges: Vec<(usize, usize)>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut used = vec![false; n];
        let mut edges = vec![];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((Reverse(0), i, i));
            }
            heap.push((Reverse(INF), i, i));
        }
        while let Some((Reverse(d), target, prev)) = heap.pop() {
            if used[target] {
                continue;
            }
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            used[target] = true;
            if target != init {
                edges.push((prev, target));
            }
            for &(next, cost) in &edge[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next, target));
                }
            }
        }
        Self { edges }
    }
}
