use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

fn main() {
    input! {
       n:usize, m:usize,
       x:Usize1, y:Usize1,
       abtk:[(Usize1,Usize1,usize,usize);m],
    }
    let mut path = vec![vec![]; n];
    for (a, b, t, k) in abtk {
        path[a].push((b, t, k));
        path[b].push((a, t, k));
    }
    let d = Dijkstra::new(n, path, x);
    println!(
        "{}",
        if d.distance(y) == INF {
            -1
        } else {
            d.distance(y) as i64
        }
    );
}

struct Dijkstra {
    distance: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edge: Vec<Vec<(usize, usize, usize)>>, init: usize) -> Self {
        let mut distance = vec![INF; n];
        let mut heap = BinaryHeap::new();
        for i in 0..n {
            if i == init {
                heap.push((Reverse(0), i));
            }
            heap.push((Reverse(INF), i));
        }
        while let Some((Reverse(d), target)) = heap.pop() {
            if distance[target] < d {
                continue;
            }
            distance[target] = d;
            for &(next, time, interval) in &edge[target] {
                let nn = ((d + interval - 1) / interval) * interval + time;
                if distance[next] > nn {
                    distance[next] = nn;
                    heap.push((Reverse(distance[next]), next));
                }
            }
        }
        Self { distance }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }
}
