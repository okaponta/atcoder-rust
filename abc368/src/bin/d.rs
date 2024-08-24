use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        k:usize,
        ab:[(Usize1, Usize1);n-1],
        v:[Usize1;k],
    }
    if k == 1 {
        println!("1");
        return;
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut lca = LCA::new(n);
    lca.init(&g, 0);
    let mut set = HashSet::new();
    for i in 0..k - 1 {
        let l = lca.query(v[i], v[i + 1]);
        set.insert(l);
        let mut tmp = v[i];
        while tmp != l {
            if set.contains(&tmp) {
                let mut next = 0;
                loop {
                    if n < lca.parent[next][tmp] {
                        break;
                    }
                    if !set.contains(&lca.parent[next][tmp]) {
                        break;
                    }
                    if lca.query(lca.parent[next][tmp], l) != l {
                        break;
                    }
                    next += 1;
                }
                tmp = lca.parent[next.max(1) - 1][tmp];
            } else {
                set.insert(tmp);
                tmp = lca.parent[0][tmp];
            }
        }
        tmp = v[i + 1];
        while tmp != l {
            if set.contains(&tmp) {
                let mut next = 0;
                loop {
                    if n < lca.parent[next][tmp] {
                        break;
                    }
                    if !set.contains(&lca.parent[next][tmp]) {
                        break;
                    }
                    if lca.query(lca.parent[next][tmp], l) != l {
                        break;
                    }
                    next += 1;
                }
                tmp = lca.parent[next.max(1) - 1][tmp];
            } else {
                set.insert(tmp);
                tmp = lca.parent[0][tmp];
            }
        }
    }
    println!("{}", set.len())
}

// LCA (Lowest Common Ancestor)
// 実装スキル不足でnewしたあとにinitをしなきゃいけない構造になってしまった・・・
pub struct LCA {
    n: usize,
    k: usize,
    // とある頂点の1,2,4...個先の頂点を保持
    // parent[0][i]...iの頂点の1つ祖先
    parent: Vec<Vec<usize>>,
    dist: Vec<usize>,
}

impl LCA {
    pub fn new(n: usize) -> Self {
        const INF: usize = 1 << 60;
        let mut k = 1;
        while 1 << k < n {
            k += 1;
        }
        let parent = vec![vec![INF; n]; k + 2];
        let dist = vec![INF; n];
        Self { n, k, parent, dist }
    }

    pub fn init(&mut self, edges: &Vec<Vec<usize>>, init: usize) {
        const INF: usize = 1 << 60;
        self.dfs(init, init, edges);
        for i in 0..self.k - 1 {
            for j in 0..self.n {
                if self.parent[i][j] == INF {
                    self.parent[i + 1][j] = INF;
                } else {
                    self.parent[i + 1][j] = self.parent[i][self.parent[i][j]];
                }
            }
        }
    }

    // uとvのLCAを求める
    pub fn query(&self, mut u: usize, mut v: usize) -> usize {
        // uを深い方として計算する
        if self.dist[u] < self.dist[v] {
            std::mem::swap(&mut u, &mut v);
        }
        // LCAまでの距離を同じにする
        for i in 0..self.k {
            if (self.dist[u] - self.dist[v]) >> i & 1 == 1 {
                u = self.parent[i][u];
            }
        }
        if u == v {
            return u;
        }
        // 一定以上の祖先は同じはずなので、上のbitから検証して揃えていく
        for i in (0..self.k).rev() {
            if self.parent[i][u] != self.parent[i][v] {
                u = self.parent[i][u];
                v = self.parent[i][v];
            }
        }
        return self.parent[0][u];
    }

    // 2点間の距離
    pub fn get_dist(&self, u: usize, v: usize) -> usize {
        self.dist[u] + self.dist[v] - 2 * self.dist[self.query(u, v)]
    }

    // 2点間のパス上に点pが存在するか判定
    pub fn is_on_path(&self, u: usize, v: usize, p: usize) -> bool {
        self.get_dist(u, p) + self.get_dist(p, v) == self.get_dist(u, v)
    }

    // 根からの距離を求める
    fn dfs(&mut self, prev: usize, cur: usize, edges: &Vec<Vec<usize>>) {
        for &next in &edges[cur] {
            if next == prev {
                continue;
            }
            self.dist[next] = self.dist[cur] + 1;
            self.parent[0][next] = cur;
            self.dfs(cur, next, edges);
        }
    }
}
