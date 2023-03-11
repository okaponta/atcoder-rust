use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        abcd:[(Usize1,char,Usize1,char);m],
    }
    let mut uf = UnionFind::new(2 * n);
    for i in 0..n {
        uf.union(i, n + i);
    }
    for &(a, b, c, d) in &abcd {
        let x = if b == 'R' { a } else { n + a };
        let y = if d == 'R' { c } else { n + c };
        uf.union(x, y);
    }
    let mut map = HashMap::new();
    for i in 0..2 * n {
        map.insert(uf.root(i), 0);
    }
    for (a, b, _, _) in abcd {
        let x = if b == 'R' { a } else { n + a };
        *map.entry(uf.root(x)).or_insert(0) += 1;
    }
    let mut x = 0;
    let mut y = 0;
    for (k, v) in map {
        if uf.size(k) == uf.size(k) / 2 + v {
            x += 1;
        } else {
            y += 1;
        }
    }
    println!("{} {}", x, y);
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
