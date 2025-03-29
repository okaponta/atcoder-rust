#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        abt:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    for (a, b, t) in abt {
        g[a].push((b, t));
        g[b].push((a, t));
    }
    let wf = WarshallFloyd::new(n, &g);
    let mut ans = 1 << 60;
    for i in 0..n {
        let mut tmp = 0;
        for j in 0..n {
            if i == j {
                continue;
            }
            tmp = tmp.max(wf.distance[i][j]);
        }
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}

// 計算量はN^3
// 負の場合でも使用でき、任意の点の最短距離がすべて求まる
pub struct WarshallFloyd {
    n: usize,
    distance: Vec<Vec<usize>>,
}

impl WarshallFloyd {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,i64)>> edges[i] = [(2,3), (3,-1), (To,重み)]
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];
        for i in 0..n {
            distance[i][i] = 0;
            for &(j, c) in &edges[i] {
                distance[i][j] = c;
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                }
            }
        }
        Self { n, distance }
    }

    pub fn update(&mut self, u: usize, v: usize, c: usize) {
        for i in 0..self.n {
            for j in 0..self.n {
                if i == j {
                    continue;
                }
                self.distance[i][j] = self.distance[i][j]
                    .min(self.distance[i][u] + c + self.distance[v][j])
                    .min(self.distance[i][v] + c + self.distance[u][j])
            }
        }
    }
}
