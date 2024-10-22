#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
    }
    let d = Dijkstra::new(n, &g).size;
    if d == 1 << 60 {
        println!("-1");
    } else {
        println!("{d}");
    }
}

pub struct Dijkstra {
    size: usize,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    pub fn new(n: usize, edges: &Vec<Vec<usize>>) -> Self {
        const INF: usize = 1 << 60;
        let mut distance = vec![INF; n];
        let mut heap = std::collections::BinaryHeap::new();
        let mut size = INF;
        for i in 0..n {
            if i == 0 {
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
                if next == 0 {
                    size = size.min(d + 1);
                }
                if distance[next] > d + 1 {
                    distance[next] = d + 1;
                    heap.push((std::cmp::Reverse(distance[next]), next));
                }
            }
        }
        Self { size }
    }
}
