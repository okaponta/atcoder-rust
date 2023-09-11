use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        l:usize,
        abc:[(Usize1,Usize1,usize);m],
        q:usize,
        st:[(Usize1,Usize1);q],
    }
    let mut g = vec![vec![]; n];
    for (a, b, c) in abc {
        if l < c {
            continue;
        }
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let mut d = vec![];
    for i in 0..n {
        d.push(Dijkstra::new(n, &g, i, l));
    }
    for (s, t) in st {
        println!("{}", d[s].ans(t));
    }
}

// 計算量は(E+V)logV
pub struct Dijkstra {
    l: usize,
    distance: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>, init: usize, l: usize) -> Self {
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
                    if l < d + cost - (d / l) * l {
                        distance[next] = ((d + cost) / l) * l + cost;
                    } else {
                        distance[next] = d + cost;
                    }
                    heap.push((std::cmp::Reverse(distance[next]), next));
                }
            }
        }
        Self { l, distance }
    }

    pub fn ans(&self, target: usize) -> i64 {
        if self.distance[target] == 1 << 60 {
            return -1;
        }
        ((self.distance[target] - 1) / self.l) as i64
    }
}
