use proconio::{fastout, input};
use std::mem::swap;

const MOD: usize = 998244353;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        abc: [(usize, usize, usize); q],
    }
    let mut prev = 0;

    let mut uf = UnionFind::new(n);
    let mut edge = vec![vec![]; n];
    let mut parent = (0..n).map(|i| i + n).collect::<Vec<_>>();

    for (a, b, c) in abc {
        let t = a * (1 + prev) % MOD % 2;
        let mut u = b * (1 + prev) % MOD % n;
        let mut v = c * (1 + prev) % MOD % n;
        if t == 0 {
            edge[u].push(v);
            edge[v].push(u);

            // 大きい方にくっつける
            if uf.size(u) > uf.size(v) {
                swap(&mut u, &mut v);
            }

            uf.union(u, v);
            dfs(&mut parent, &edge, u, v);
        } else {
            prev = 0;

            if parent[u] == parent[v] {
                prev = parent[u] + 1;
            }

            if parent[u] < n && parent[parent[u]] == v {
                prev = parent[u] + 1;
            }

            if parent[v] < n && parent[parent[v]] == u {
                prev = parent[v] + 1;
            }

            println!("{}", prev);
        }
    }
}

fn dfs(parent: &mut Vec<usize>, edge: &Vec<Vec<usize>>, u: usize, v: usize) {
    parent[u] = v;

    for &w in edge[u].iter() {
        if w != v {
            dfs(parent, edge, w, u);
        }
    }
}
pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            rank: vec![0; n],
        }
    }

    // 根を返却
    pub fn root(&mut self, x: usize) -> usize {
        // parentが自分自身の場合は根
        if self.parent[x] == x {
            return x;
        }
        // 経路圧縮
        self.parent[x] = self.root(self.parent[x]);
        self.parent[x]
    }

    // xとyが同じ根か判定
    pub fn equiv(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // xとyを合体
    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut rx = self.root(x);
        let mut ry = self.root(y);
        // 既に同じ
        if rx == ry {
            return false;
        }

        // ryのrankが小さくなるように調整
        // ここを省略するとrxが親になる
        if self.rank[rx] < self.rank[ry] {
            std::mem::swap(&mut rx, &mut ry);
        }
        // ryの根をrxにする
        self.parent[ry] = rx;
        // rxのrank調整
        if self.rank[rx] == self.rank[ry] {
            self.rank[rx] += 1;
        }
        self.size[rx] += self.size[ry];
        true
    }

    // xのグループの要素数
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.size[root]
    }

    // 連結かどうかを返却する
    pub fn is_linked(&mut self) -> bool {
        self.size(0) == self.size.len()
    }
}
