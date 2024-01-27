use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uvw:[(Usize1,Usize1,i64);m],
    }
    let mut g = vec![vec![]; n];
    for (u, v, w) in uvw {
        g[u].push((v, w));
    }
    let mut d = vec![vec![(0, 0); n]; n];
    let mut to_all = vec![true; n];
    for i in 0..n {
        let dij = Dijkstra::new(n, &g, i);
        for j in 0..n {
            if i != j {
                d[i][j] = (dij.distance(j), dij.get_path(j));
                if dij.distance(j) == 1 << 60 {
                    to_all[i] = false;
                }
            }
        }
    }
    if to_all.iter().all(|b| !b) {
        println!("No");
        return;
    }
    let mut dp = vec![vec![1 << 60; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }
    for i in 1..1 << n {
        for j in 0..n {
            if i >> j & 1 == 0 {
                // 未訪問
                continue;
            }
            for k in 0..n {
                if j == k {
                    continue;
                }
                if i >> k & 1 == 1 {
                    // 訪問済み
                    continue;
                }
                let tmp = d[j][k];
                dp[i | tmp.1][k] = dp[i | tmp.1][k].min(dp[i][j] + tmp.0);
            }
        }
    }
    let mut ans = 1 << 60;
    for i in 0..n {
        let tmp = dp[(1 << n) - 1][i];
        let tmp2 = (0..n)
            .into_iter()
            .map(|j| d[i][j].0)
            .min()
            .unwrap_or(0)
            .min(0);
        ans = ans.min(tmp + tmp2);
    }
    println!("{}", ans);
}

// 計算量は(E+V)logV
pub struct Dijkstra {
    distance: Vec<i64>,
    parent: Vec<usize>,
}

impl Dijkstra {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edge[i] = [(2,3), (3,1), (頂点への道,重み)]
    // init:usize どの頂点を起点に考えるか
    pub fn new(n: usize, edges: &Vec<Vec<(usize, i64)>>, init: usize) -> Self {
        const INF: i64 = 1 << 60;
        let mut distance = vec![INF; n];
        let mut parent = vec![INF as usize; n];
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

    pub fn distance(&self, target: usize) -> i64 {
        self.distance[target]
    }

    pub fn get_path(&self, target: usize) -> usize {
        const INF: usize = 1 << 60;
        let mut current = target;
        let mut res = 1 << current;
        while self.parent[current] != INF as usize {
            let next = self.parent[current];
            res |= 1 << next;
            current = next;
        }
        res
    }
}
