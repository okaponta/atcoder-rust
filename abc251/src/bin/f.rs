use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut edges = vec![vec![]; n];
    for (u, v) in uv {
        edges[u].push(v);
        edges[v].push(u);
    }

    // dfs tree
    let mut vis = vec![false; n];
    let mut dfs_edges = vec![];
    vis[0] = true;
    dfs(0, &edges, &mut vis, &mut dfs_edges);
    // bfs tree
    vis = vec![false; n];
    vis[0] = true;
    let mut bfs_edges = vec![];
    let mut q = VecDeque::new();
    q.push_back(0);
    while let Some(cur) = q.pop_front() {
        for &next in &edges[cur] {
            if vis[next] {
                continue;
            }
            vis[next] = true;
            bfs_edges.push((cur, next));
            q.push_back(next);
        }
    }
    for (u, v) in dfs_edges {
        println!("{} {}", u + 1, v + 1);
    }
    for (u, v) in bfs_edges {
        println!("{} {}", u + 1, v + 1);
    }
}

fn dfs(cur: usize, edges: &Vec<Vec<usize>>, vis: &mut Vec<bool>, ans: &mut Vec<(usize, usize)>) {
    for &next in &edges[cur] {
        if vis[next] {
            continue;
        }
        vis[next] = true;
        ans.push((cur, next));
        dfs(next, edges, vis, ans);
    }
}
