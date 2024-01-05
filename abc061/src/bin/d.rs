use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        abc:[(Usize1,Usize1,i64);m],
    }
    let edges = abc
        .into_iter()
        .map(|(a, b, c)| (a, b, -c))
        .collect::<Vec<_>>();
    let g = BellmanFord::new(n, edges, 0);
    if g.negative[n - 1] {
        println!("inf");
        return;
    }
    println!("{}", -g.distance(n - 1));
}

// 計算量はE×V
#[allow(dead_code)]
pub struct BellmanFord {
    distance: Vec<i64>,
    has_neg_loop: bool,
    negative: Vec<bool>,
}

impl BellmanFord {
    // n:usize 頂点の数
    // edges: Vec<(usize,usize,i64)> edges[i] = [(0,2,3), (1,3,-1), (From,To,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: Vec<(usize, usize, i64)>, init: usize) -> Self {
        let mut distance = vec![1 << 60; n];
        distance[init] = 0;
        let mut has_neg_loop = false;

        for i in 0..n {
            for edge in &edges {
                let from = edge.0;
                let to = edge.1;
                let cost = edge.2;
                if distance[to] > distance[from] + cost {
                    distance[to] = distance[from] + cost;
                    if i == n - 1 {
                        has_neg_loop = true;
                        break;
                    }
                }
            }
        }

        let mut negative = vec![false; n];
        if !has_neg_loop {
            return Self {
                distance,
                has_neg_loop,
                negative,
            };
        }

        for _ in 0..n {
            for edge in &edges {
                let from = edge.0;
                let to = edge.1;
                let cost = edge.2;
                if distance[to] > distance[from] + cost {
                    negative[to] = true;
                }
                if negative[from] {
                    negative[to] = true;
                }
            }
        }

        Self {
            distance,
            has_neg_loop,
            negative,
        }
    }

    pub fn distance(&self, target: usize) -> i64 {
        self.distance[target]
    }
}
