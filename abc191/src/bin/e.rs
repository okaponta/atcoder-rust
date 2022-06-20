use proconio::{fastout, input, marker::Usize1};

const INF: i64 = 1 << 60;

#[fastout]
fn main() {
    input! {
       n:usize,
       m:usize,
       abc:[(Usize1,Usize1,i64);m],
    }
    let mut edges = vec![vec![]; n];
    let mut ans = vec![INF; n];
    for (a, b, c) in abc {
        if a == b {
            ans[a] = ans[a].min(c);
        } else {
            edges[a].push((b, c));
        }
    }
    let mut dists = vec![];
    for i in 0..n {
        let d = Dijkstra::new(n, &edges, i);
        dists.push(d.distance);
    }
    for i in 0..n {
        for j in 0..n {
            if j == i {
                continue;
            }
            ans[i] = ans[i].min(dists[i][j] + dists[j][i]);
        }
        println!("{}", if ans[i] == INF { -1 } else { ans[i] });
    }
}

pub struct Dijkstra {
    distance: Vec<i64>,
}

impl Dijkstra {
    pub fn new(n: usize, edges: &Vec<Vec<(usize, i64)>>, init: usize) -> Self {
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
}
