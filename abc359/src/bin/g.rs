use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n-1],
        a:[usize;n],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let map = dfs(0, 0, 0, &g, &a);
    println!("{}", map.values().map(|r| r.1).sum::<usize>());
}

fn dfs(
    prev: usize,
    cur: usize,
    dep: usize,
    g: &Vec<Vec<usize>>,
    a: &Vec<usize>,
) -> HashMap<usize, (usize, usize, usize)> {
    let mut res = HashMap::new();
    res.insert(a[cur], (dep, 0, 1));
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        let mut ret = dfs(cur, next, dep + 1, g, a);
        if res.len() < ret.len() {
            std::mem::swap(&mut res, &mut ret)
        }
        for (k, v) in &ret {
            res.insert(*k, f(res.get(k).unwrap_or(&(0, 0, 0)), v, dep));
        }
    }
    res
}

fn f(a: &(usize, usize, usize), b: &(usize, usize, usize), d: usize) -> (usize, usize, usize) {
    (
        a.0 + b.0,
        a.1 + b.1 + a.0 * b.2 + a.2 * b.0 - a.2 * b.2 * 2 * d,
        a.2 + b.2,
    )
}
