use proconio::{fastout, input, marker::Usize1};

const EMPTY: usize = 1 << 60;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut uf = UnionFind::new(n + q);
    let mut lead = (0..n).into_iter().collect::<Vec<_>>();
    let mut rev = (0..n + q).into_iter().collect::<Vec<_>>();
    let mut max = n;
    for _ in 0..q {
        input! {q: u8, x:Usize1}
        if q == 1 {
            input! {y: Usize1}
            let rx = lead[x];
            let ry = lead[y];
            if ry != EMPTY {
                if rx == EMPTY {
                    lead[x] = ry;
                } else {
                    uf.union(rx, ry);
                }
                rev[uf.root(ry)] = x;
                lead[y] = EMPTY;
            }
        } else if q == 2 {
            let rx = lead[x];
            if rx != EMPTY {
                uf.union(rx, max);
            } else {
                lead[x] = max;
            }
            rev[uf.root(max)] = x;
            max += 1;
        } else {
            println!("{}", rev[uf.root(x)] + 1);
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
