use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        abc:[(Usize1,Usize1,usize);n-1],
        q:usize,
        k:Usize1,
        xy:[(Usize1,Usize1);q],
    }
    let mut g = vec![vec![]; n];
    for (a, b, c) in abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let d = Dijkstra::new(n, &g, k);
    for (x, y) in xy {
        println!("{}", d.distance(x) + d.distance(y));
    }
}

pub struct Dijkstra {
    distance: Vec<usize>,
}

impl Dijkstra {
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
