use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    let mut count = vec![0; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
        count[u] += 1;
        count[v] += 1;
    }
    let mut used = vec![false; n];
    let first = (0..n).into_iter().find(|&i| count[i] == 1).unwrap();
    let mut q = VecDeque::new();
    let mut ans = vec![];
    q.push_back(first);
    while let Some(cur) = q.pop_front() {
        for &center in &g[cur] {
            if used[center] {
                continue;
            }
            ans.push(count[center]);
            used[center] = true;
            for &node in &g[center] {
                if count[node] != 1 {
                    for &next in &g[node] {
                        if used[next] {
                            continue;
                        }
                        q.push_back(next);
                    }
                }
                used[node] = true;
            }
        }
    }
    ans.sort();
    println!("{}", ans.iter().join(" "));
}
