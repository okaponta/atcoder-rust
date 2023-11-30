use std::{cmp::Reverse, collections::BinaryHeap};

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n],
    }
    let mut x = vec![];
    let mut y = vec![];
    for i in 0..n {
        x.push((xy[i].0, i));
        y.push((xy[i].1, i));
    }
    x.sort();
    y.sort();
    let mut heap = BinaryHeap::new();
    for i in 0..n - 1 {
        heap.push(Reverse((x[i + 1].0 - x[i].0, x[i + 1].1, x[i].1)));
        heap.push(Reverse((y[i + 1].0 - y[i].0, y[i + 1].1, y[i].1)));
    }
    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    while let Some(Reverse((d, u, v))) = heap.pop() {
        if uf.union(u, v) {
            ans += d;
        }
    }

    println!("{}", ans);
}
