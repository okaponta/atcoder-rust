use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        ab:[(Usize1,Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        uf.union(a, b);
    }
    let mut size = HashMap::new();
    for i in 0..n {
        *size.entry(uf.find(i)).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (_, v) in size {
        ans += v * (v - 1) / 2;
    }
    println!("{}", ans - m);
}
