use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut a = vec![];
    let mut c = vec![];
    for i in 0..m {
        input! {
            k:usize,
            ci:usize,
            ai:[Usize1;k],
        }
        a.push(ai);
        c.push((ci, i));
    }
    c.sort();
    let mut ans = 0;
    let mut uf = UnionFind::new(n);
    for (c, i) in c {
        for j in 1..a[i].len() {
            if uf.union(a[i][0], a[i][j]) {
                ans += c;
            }
        }
    }
    if uf.is_linked() {
        println!("{}", ans);
    } else {
        println!("-1");
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
