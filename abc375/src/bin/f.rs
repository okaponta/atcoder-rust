use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        q:usize,
        abc:[(Usize1,Usize1,usize);m],
    }
    let mut query = vec![];
    let mut set = (0..m).into_iter().collect::<HashSet<_>>();
    for _ in 0..q {
        input! {qi: u8}
        if qi == 1 {
            input! {i: Usize1}
            query.push((qi, i, 0));
            set.remove(&i);
        } else {
            input! {x:Usize1, y:Usize1}
            query.push((qi, x, y));
        }
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        if set.contains(&i) {
            g[abc[i].0].push((abc[i].1, abc[i].2));
            g[abc[i].1].push((abc[i].0, abc[i].2));
        }
    }
    let mut w = WarshallFloyd::new(n, &g);
    let mut ans = vec![];
    for (qi, x, y) in query.into_iter().rev() {
        if qi == 1 {
            w.update(abc[x].0, abc[x].1, abc[x].2);
        } else {
            ans.push(w.distance[x][y]);
        }
    }
    for &ans in ans.iter().rev() {
        if (1 << 59) < ans {
            println!("-1");
        } else {
            println!("{}", ans);
        }
    }
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
