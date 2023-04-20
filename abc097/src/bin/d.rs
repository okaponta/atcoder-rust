use std::collections::{HashMap, HashSet};

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        p:[Usize1;n],
        xy:[(Usize1,Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    for (x, y) in xy {
        uf.union(x, y);
    }
    let mut map = HashMap::new();
    for i in 0..n {
        (*map.entry(uf.find(i)).or_insert(HashSet::new())).insert(i);
    }
    println!(
        "{}",
        (0..n)
            .into_iter()
            .filter(|&i| map[&uf.find(i)].contains(&p[i]))
            .count()
    );
}
