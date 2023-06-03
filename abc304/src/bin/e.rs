use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
        k:usize,
        xy:[(Usize1,Usize1);k],
        q:usize,
        pq:[(Usize1,Usize1);q]
    }
    let mut uf = UnionFind::new(n);
    for (u, v) in uv {
        uf.union(u, v);
    }
    let mut set = HashSet::new();
    for (x, y) in xy {
        let a = uf.find(x);
        let b = uf.find(y);
        set.insert((a.min(b), a.max(b)));
    }
    for (p, q) in pq {
        let a = uf.find(p);
        let b = uf.find(q);
        println!(
            "{}",
            if set.contains(&(a.min(b), a.max(b))) {
                "No"
            } else {
                "Yes"
            }
        )
    }
}
