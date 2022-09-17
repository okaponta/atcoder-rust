use std::collections::{HashMap, HashSet};

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let mut set = HashSet::new();
    let mut map = HashMap::new();
    let mut uf = UnionFind::new(n);
    let mut rep = vec![0; n];
    let dx = vec![-1, -1, 0, 0, 1, 1];
    let dy = vec![-1, 0, -1, 1, 0, 1];
    for (i, &(x, y)) in xy.iter().enumerate() {
        map.insert((x, y), i);
        set.insert((x, y));
        for j in 0..6 {
            let nx = x + dx[j];
            let ny = y + dy[j];
            if set.contains(&(nx, ny)) {
                uf.union(i, map[&(nx, ny)]);
            }
        }
    }
    for i in 0..n {
        rep[i] = uf.find(i);
    }
    rep.sort();
    rep.dedup();
    println!("{}", rep.len());
}
