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
        uvt:[(Usize1,Usize1,usize);m],
        q:usize,
    }
    let mut g = vec![vec![]; n];
    for &(u, v, t) in &uvt {
        g[u].push((v, t));
        g[v].push((u, t));
    }
    let w = WarshallFloyd::new(n, &g);
    for _ in 0..q {
        input! {
            k:usize,
            b:[Usize1;k],
        }
        let mut dp = vec![vec![(1 << 60, 1 << 60); k]; 1 << k];
        let mut v = vec![];
        for &b in &b {
            v.push((uvt[b].0, uvt[b].1));
        }
        for i in 0..k {
            dp[0][i] = (w.d[0][v[i].0], w.d[0][v[i].1]);
        }
        for i in 0..1 << k {
            // from
            for j in 0..k {
                // to
                for l in 0..k {
                    if i >> l & 1 == 1 {
                        // 訪問済
                        continue;
                    }
                    let (ui, vi, ti) = uvt[b[l]];
                    dp[i | 1 << l][l].0 = dp[i | 1 << l][l]
                        .0
                        .min(dp[i][j].0 + w.d[v[j].0][vi] + ti)
                        .min(dp[i][j].1 + w.d[v[j].1][vi] + ti);
                    dp[i | 1 << l][l].1 = dp[i | 1 << l][l]
                        .1
                        .min(dp[i][j].0 + w.d[v[j].0][ui] + ti)
                        .min(dp[i][j].1 + w.d[v[j].1][ui] + ti);
                }
            }
        }
        let mut ans = 1 << 60;
        for i in 0..k {
            ans = ans.min(dp[(1 << k) - 1][i].0 + w.d[v[i].0][n - 1]);
            ans = ans.min(dp[(1 << k) - 1][i].1 + w.d[v[i].1][n - 1]);
        }
        println!("{}", ans);
    }
}

// 計算量はN^3
// 負の場合でも使用でき、任意の点の最短距離がすべて求まる
pub struct WarshallFloyd {
    d: Vec<Vec<usize>>,
}

impl WarshallFloyd {
    // n:usize 頂点の数
    // edges: Vec<Vec<(usize,usize)>> edges[i] = [(2,3), (3,-1), (To,重み)]
    pub fn new(n: usize, edges: &Vec<Vec<(usize, usize)>>) -> Self {
        let mut distance = vec![vec![1 << 60; n]; n];
        for i in 0..n {
            for &(j, c) in &edges[i] {
                distance[i][j] = distance[i][j].min(c);
            }
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    distance[i][j] = distance[i][j].min(distance[i][k] + distance[k][j]);
                    // 普通に足す場合は足してこの場合はmaxをとる問題だった(こんなパターンもあるよ紹介)
                    // distance[i][j] = distance[i][j].min(distance[i][k].max(distance[k][j]));
                }
            }
        }
        for i in 0..n {
            distance[i][i] = 0;
        }
        Self { d: distance }
    }
}
