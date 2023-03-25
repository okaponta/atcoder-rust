use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        p:[Usize1;n-1],
        q:usize,
    }
    let mut g = vec![vec![]; n];
    let mut rev_g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[p[i]].push(i + 1);
        rev_g[i + 1].push(p[i]);
    }
    let mut uf = UnionFind::new(n);
    let mut ans = (0..n).collect::<Vec<_>>();
    for _ in 0..q {
        input! {q: u8}
        if q == 1 {
            input! {u: Usize1, v:Usize1}
            dfs(u, u, v, &rev_g, &mut uf, &mut ans);
        } else {
            input! {x:Usize1}
            println!("{}", ans[uf.root(x)] + 1);
        }
    }
}

fn dfs(
    cur: usize,
    prev: usize,
    goal: usize,
    g: &Vec<Vec<usize>>,
    uf: &mut UnionFind,
    ans: &mut Vec<usize>,
) {
    uf.union(prev, cur);
    if ans[cur] <= goal {
        // 続けなくてOK
        return;
    }
    ans[cur] = goal;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, goal, g, uf, ans);
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

        // rxの方が小さくする
        if ry < rx {
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
