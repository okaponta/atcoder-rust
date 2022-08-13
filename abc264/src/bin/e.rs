use std::collections::HashSet;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,_:usize,e:usize,
        uv:[(Usize1,Usize1);e],
        q:usize,
        mut x:[Usize1;q],
    }
    let mut x_set = HashSet::new();
    for i in 0..q {
        x_set.insert(x[i]);
    }
    let mut uf = UnionFind::new(n + 1);
    for i in 0..e {
        if x_set.contains(&i) {
            continue;
        }
        let u = uv[i].0.min(n);
        let v = uv[i].1.min(n);
        uf.unite(u, v);
    }
    x.reverse();
    let mut ans = vec![];
    for x in x {
        ans.push(uf.size(n) - 1);
        let u = uv[x].0.min(n);
        let v = uv[x].1.min(n);
        uf.unite(u, v);
    }
    for i in (0..q).rev() {
        println!("{}", ans[i]);
    }
}

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]);
        self.par[x]
    }

    // fn issame(&mut self, x: usize, y: usize) -> bool {
    //     self.root(x) == self.root(y)
    // }

    fn unite(&mut self, mut parent: usize, mut child: usize) -> bool {
        parent = self.root(parent);
        child = self.root(child);

        if parent == child {
            return false;
        }

        if self.siz[parent] < self.siz[child] {
            std::mem::swap(&mut parent, &mut child);
        }

        self.par[child] = parent;
        self.siz[parent] += self.siz[child];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.siz[root]
    }
}
