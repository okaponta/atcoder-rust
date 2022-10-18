use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        xyz:[(Usize1,Usize1,usize);m],
    }
    let mut uf = UnionFind::new(n);
    for (x, y, _) in xyz {
        uf.union(x, y);
    }
    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(uf.find(i));
    }
    println!("{}", set.len());
}
