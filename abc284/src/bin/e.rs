use std::process::exit;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut used = vec![false; n];
    let mut count = 1;
    dfs(0, &mut used, &g, &mut count);
    println!("{}", count);
}

fn dfs(cur: usize, used: &mut Vec<bool>, g: &Vec<Vec<usize>>, count: &mut usize) {
    if *count >= 1_000_000 {
        println!("1000000");
        exit(0);
    }
    used[cur] = true;
    for &next in &g[cur] {
        if used[next] {
            continue;
        }
        *count += 1;
        dfs(next, used, g, count);
        used[next] = false;
    }
}
