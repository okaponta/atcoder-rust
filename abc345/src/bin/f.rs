use std::collections::{BinaryHeap, HashSet};

use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        uv:[(Usize1,Usize1);m],
    }
    if k % 2 == 1 {
        println!("No");
        return;
    }
    if k == 0 {
        println!("Yes");
        println!("0");
        return;
    }
    let mut uf = UnionFind::new(n);
    let mut g = vec![vec![]; n];
    let mut ng = vec![false; m];
    for i in 0..m {
        let (u, v) = uv[i];
        if uf.union(u, v) {
            g[u].push(v);
            g[v].push(u);
        } else {
            ng[i] = true;
        }
    }
    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(uf.find(i));
    }
    let mut depth = vec![0; n];
    for i in set {
        dfs(i, i, 0, &mut depth, &g);
    }
    let mut up = vec![0; n];
    for i in 0..m {
        if ng[i] {
            continue;
        }
        if depth[uv[i].0] < depth[uv[i].1] {
            up[uv[i].1] = i;
        } else {
            up[uv[i].0] = i;
        }
    }
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push((depth[i], i));
    }
    let mut lamp = vec![false; n];
    let mut ans = vec![];
    let mut count = 0;
    while let Some((_, i)) = heap.pop() {
        if !lamp[i] && depth[i] != 0 {
            ans.push(up[i] + 1);
            lamp[uv[up[i]].0] = !lamp[uv[up[i]].0];
            lamp[uv[up[i]].1] = !lamp[uv[up[i]].1];
            if lamp[uv[up[i]].0] && lamp[uv[up[i]].1] {
                count += 2;
                if count == k {
                    println!("Yes");
                    println!("{}", ans.len());
                    println!("{}", ans.iter().join(" "));
                    return;
                }
            }
        }
    }
    println!("No");
}

fn dfs(cur: usize, prev: usize, d: usize, depth: &mut Vec<usize>, g: &Vec<Vec<usize>>) {
    depth[cur] = d;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, d + 1, depth, g);
    }
}
