use proconio::input;

fn main() {
    input! {
        h:usize,
        w:usize,
        c:[[usize;10];10],
        a:[[i64;w];h],
    }
    let mut g = vec![vec![]; 10];
    for i in 0..10 {
        for j in 0..10 {
            g[j].push((i, c[i][j]));
        }
    }
    let d = Dijkstra::new(10, &g, 1);
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if 0 <= a[i][j] {
                ans += d.distance[a[i][j] as usize];
            }
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
