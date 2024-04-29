use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut g = vec![vec![]; n + m];
    for i in 0..m {
        input! {
            k:usize,
            r:[Usize1;k],
        }
        for j in r {
            g[n + i].push(j);
            g[j].push(n + i);
        }
    }
    let d = Dijkstra::new(n + m, &g, 0);
    for i in 0..n {
        println!("{}", d.distance(i) / 2);
    }
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

    pub fn distance(&self, target: usize) -> i64 {
        if self.distance[target] == 1 << 60 {
            -2
        } else {
            self.distance[target] as i64
        }
    }
}
