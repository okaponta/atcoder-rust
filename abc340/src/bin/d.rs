use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        abx:[(usize,usize,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[i].push((i + 1, abx[i].0));
        g[i].push((abx[i].2, abx[i].1));
    }
    let d = Dijkstra::new(n, &g, 0);
    println!("{}", d.distance(n - 1));
}

pub struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>, init: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut distance = vec![INF; n];
        let mut parent = vec![INF; n];
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
                    parent[next] = target;
                }
            }
        }
        Self { distance, parent }
    }

    pub fn distance(&self, target: usize) -> usize {
        self.distance[target]
    }

    pub fn get_path(&self, target: usize) -> Vec<usize> {
        const INF: usize = 1 << 60;
        let mut current = target;
        let mut res = vec![current];
        while self.parent[current] != INF as usize {
            let next = self.parent[current];
            res.push(next);
            current = next;
        }
        res.reverse();
        res
    }
}
