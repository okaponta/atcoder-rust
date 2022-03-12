use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use proconio::{input, marker::Usize1};

const INF: usize = 1 << 60;

fn main() {
    input! {
        n:usize,m:usize,
        abc:[(Usize1,Usize1,usize);m],
    }
    let mut edge = vec![vec![]; n];
    for (i, &(a, b, c)) in abc.iter().enumerate() {
        edge[a].push((b, c, i));
        edge[b].push((a, c, i));
    }

    let mut set = HashSet::new();
    for i in 0..n {
        let d = Dijkstra::new(n, &edge, i, &set);
        for &(b, c, index) in &edge[i] {
            if d.distance(b) < c {
                set.insert(index);
            } else if d.distance(b) == c {
                if d.is_mul[b] {
                    set.insert(index);
                }
            }
        }
    }
    println!("{}", set.len());
}

// 計算量は(E+V)logV
struct Dijkstra {
    distance: Vec<usize>,
    is_mul: Vec<bool>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edge: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(
        n: usize,
        edge: &Vec<Vec<(usize, usize, usize)>>,
        init: usize,
        set: &HashSet<usize>,
    ) -> Self {
        let mut distance = vec![INF; n];
        let mut is_mul = vec![false; n];
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
            for &(next, cost, i) in &edge[target] {
                if set.contains(&i) {
                    continue;
                }
                if distance[next] > d + cost {
                    distance[next] = d + cost;
                    heap.push((Reverse(distance[next]), next));
                    is_mul[next] = false;
                } else if distance[next] == d + cost {
                    is_mul[next] = true;
                }
            }
        }
        Self { distance, is_mul }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }
}
