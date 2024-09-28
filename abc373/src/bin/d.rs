#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        uvw:[(Usize1,Usize1,i64);m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        g[u].push((v, w));
        g[v].push((u, -w));
    }
    let mut used = vec![false; n];
    let mut ans = vec![0; n];
    for i in 0..n {
        if used[i] {
            continue;
        }
        dfs(i, i, 0, &mut used, &mut ans, &g);
    }
    println!("{}", ans.iter().join(" "));
}

fn dfs(
    cur: usize,
    _prev: usize,
    cost: i64,
    used: &mut Vec<bool>,
    ans: &mut Vec<i64>,
    g: &Vec<Vec<(usize, i64)>>,
) {
    ans[cur] = cost;
    used[cur] = true;
    for &next in &g[cur] {
        if used[next.0] {
            continue;
        }
        dfs(next.0, cur, cost + next.1, used, ans, g);
    }
}
