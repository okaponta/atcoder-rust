use num_integer::Roots;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,
       m:usize,
       abcd:[(Usize1,Usize1,usize,usize);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b, c, d) in abcd {
        g[a].push((b, c, d));
        g[b].push((a, c, d));
    }
    let d = Dijkstra::new(n, &g, 0);
    if d.distance[n - 1] == 1 << 60 {
        println!("-1");
    } else {
        println!("{}", d.distance[n - 1]);
    }
}

pub struct Dijkstra {
    distance: Vec<usize>,
}

impl Dijkstra {
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize, usize)>>, init: usize) -> Self {
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
            for &(next, ci, di) in &edges[target] {
                let arrive = f(d, ci, di);
                if distance[next] > arrive {
                    distance[next] = arrive;
                    heap.push((std::cmp::Reverse(distance[next]), next));
                }
            }
        }
        Self { distance }
    }
}

fn f(t: usize, c: usize, d: usize) -> usize {
    if d == 0 {
        return dist(t, d) + c;
    }
    if t <= d.sqrt() - 1 {
        return (dist(d.sqrt() - 1, d)).min(dist(d.sqrt(), d)) + c;
    } else {
        return dist(t, d) + c;
    }
}

fn dist(t: usize, d: usize) -> usize {
    t + d / (t + 1)
}
