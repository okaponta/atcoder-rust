use std::collections::BTreeSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        uv:[(Usize1,Usize1);m],
    }
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((a[i], i));
    }
    let mut uf = UnionFind::new(n);
    for &(u, v) in &uv {
        if a[u] == a[v] {
            uf.union(u, v);
        }
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        let u = uf.find(u);
        let v = uf.find(v);
        if a[u] < a[v] {
            g[u].push(v);
        }
        if a[v] < a[u] {
            g[v].push(u);
        }
    }
    let mut ans = vec![0usize; n];
    let mut used = vec![false; n];
    ans[uf.find(0)] = 1;
    for (_, v) in set {
        if used[uf.find(v)] {
            continue;
        }
        used[uf.find(v)] = true;
        let cur = uf.find(v);
        if ans[cur] == 0 {
            continue;
        }
        for &next in &g[cur] {
            ans[next] = ans[next].max(ans[cur] + 1);
        }
    }
    println!("{}", ans[uf.find(n - 1)]);
}
