use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uvl:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    for (u, v, l) in uvl {
        g[u].push((v, l));
        g[v].push((u, l));
    }
    let mut ans = 1 << 60;
    for _i in 0..g[0].len() {
        let tmp = g[0][0];
        g[0].remove(0);
        let d = Dijkstra::new(n, &g, 0);
        ans = ans.min((d.distance(tmp.0)) + tmp.1);
        g[0].push(tmp);
    }
    if 1 << 50 < ans {
        println!("-1");
    } else {
        println!("{ans}")
    }
}

pub struct Dijkstra {
    distance: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>, init: usize) -> Self {
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
            for &(next, cost) in &edges[target] {
                if distance[next] > d + cost {
                    distance[next] = d + cost;
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
