use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        abd:[(Usize1,Usize1,i64);q],
    }
    let mut uf = UnionFind::new(n);
    let mut g = vec![vec![]; n];
    for i in 0..q {
        if uf.union(abd[i].0, abd[i].1) {
            g[abd[i].0].push((abd[i].1, abd[i].2));
            g[abd[i].1].push((abd[i].0, -abd[i].2));
        }
    }
    let mut target = HashSet::new();
    for i in 0..n {
        if uf.size(i) != 1 {
            target.insert(uf.root(i));
        }
    }
    let mut num = vec![0; n];
    let mut used = vec![false; n];
    let mut queue = VecDeque::new();
    for i in target {
        queue.push_back(i);
        used[i] = true;
    }
    while let Some(cur) = queue.pop_front() {
        for &(next, cost) in &g[cur] {
            if used[next] {
                continue;
            }
            num[next] = num[cur] - cost;
            used[next] = true;
            queue.push_back(next);
        }
    }
    let mut ans = vec![];
    for i in 0..q {
        if num[abd[i].0] - num[abd[i].1] == abd[i].2 {
            ans.push(i + 1);
        }
    }
    println!("{}", ans.iter().join(" "));
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
