use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n1:usize,
        n2:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut g1 = vec![vec![]; n1];
    let mut g2 = vec![vec![]; n2];
    for (a, b) in ab {
        if a < n1 {
            g1[a].push(b);
            g1[b].push(a);
        } else {
            g2[a - n1].push(b - n1);
            g2[b - n1].push(a - n1);
        }
    }
    let d1 = Dijkstra::new(n1, &g1, 0);
    let d2 = Dijkstra::new(n2, &g2, n2 - 1);
    let ans = d1.max_dist() + d2.max_dist() + 1;
    println!("{}", ans);
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

    pub fn max_dist(&self) -> usize {
        *self.distance.iter().max().unwrap_or(&0)
    }
}
