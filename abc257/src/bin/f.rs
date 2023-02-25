use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(usize,usize);m],
    }
    let mut edges = vec![vec![]; n + 1];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }
    let d = Dijkstra::new(n + 1, &edges, 1);
    let rev_d = Dijkstra::new(n + 1, &edges, n);
    let mut ans = vec![];
    for i in 1..=n {
        let a = d.distance(n);
        let b = d.distance(0) + rev_d.distance(i);
        let c = d.distance(i) + rev_d.distance(0);
        let ansi = a.min(b).min(c);
        ans.push(if ansi == 1 << 60 { -1 } else { ansi as i64 });
    }
    println!("{}", ans.into_iter().join(" "));
}

pub struct Dijkstra {
    distance: Vec<usize>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<usize>>, init: usize) -> Self {
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
            for &next in &edges[target] {
                if distance[next] > d + 1 {
                    distance[next] = d + 1;
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
