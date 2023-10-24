use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[[usize;n];n],
    }
    let mut path = vec![];
    for i in 0..n {
        for j in i + 1..n {
            path.push((a[i][j], i, j));
        }
    }
    path.sort();
    let mut g = vec![vec![]; n];
    let mut ans = 0;
    for (c, i, j) in path {
        let d = Dijkstra::new(n, &g, i);
        if d.distance(j) < c {
            println!("-1");
            return;
        }
        if d.distance(j) != c {
            g[i].push((j, c));
            g[j].push((i, c));
            ans += c;
        }
    }
    println!("{}", ans);
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
