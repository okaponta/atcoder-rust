use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,
        ab:[(usize,usize);m],
        q:usize,
        xk:[(usize,usize);q],
    }
    let mut edges = vec![vec![]; n + 1];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    for (x, k) in xk {
        let mut vis = HashSet::new();
        vis.insert(x);
        dfs(&mut vis, x, k, &edges);
        println!("{}", vis.iter().sum::<usize>());
    }
}

fn dfs(vis: &mut HashSet<usize>, cur: usize, depth: usize, edges: &Vec<Vec<usize>>) {
    if depth == 0 {
        return;
    }
    for &next in &edges[cur] {
        vis.insert(next);
        dfs(vis, next, depth - 1, edges);
    }
}
