use std::collections::VecDeque;

use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
        txyv:[(u8,Usize1,Usize1,i64);q],
    }
    let mut uf = UnionFind::new(n);
    let mut g = vec![vec![]; n];
    let mut amb = vec![false; q];
    for (i, &(t, x, y, v)) in txyv.iter().enumerate() {
        if t == 0 {
            uf.union(x, y);
            g[x].push((y, v));
            g[y].push((x, v));
        } else {
            if !uf.equiv(x, y) {
                amb[i] = true;
            }
        }
    }
    let mut used = vec![false; n];
    let mut pot = vec![(0, 0); n];
    let mut q = VecDeque::new();
    (0..n)
        .into_iter()
        .map(|i| uf.find(i))
        .for_each(|i| q.push_back((i, 1, 0)));
    while let Some((cur, pm, vv)) = q.pop_front() {
        if used[cur] {
            continue;
        }
        used[cur] = true;
        pot[cur] = (pm, vv);
        for &(next, sum) in &g[cur] {
            if !used[next] {
                q.push_back((next, -pm, sum - vv));
            }
        }
    }
    for (i, &(t, x, y, v)) in txyv.iter().enumerate() {
        if t == 0 {
            continue;
        }
        if amb[i] {
            println!("Ambiguous");
            continue;
        }
        println!("{}", ((v - pot[x].1) * pot[x].0) * pot[y].0 + pot[y].1);
    }
}
