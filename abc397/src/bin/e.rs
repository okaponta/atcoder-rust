#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        k:usize,
        uv:[(Usize1,Usize1);n*k-1],
    }
    let mut g = vec![vec![]; n * k];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut dp = vec![0; n * k];
    dfs(0, 0, &g, &mut dp, k);
    println!("Yes");
}

fn dfs(cur: usize, prev: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<usize>, k: usize) {
    let mut c = 0;
    let mut s = 1;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, g, dp, k);
        if 0 < dp[next] {
            c += 1;
        }
        s += dp[next];
    }
    if s < k {
        if 1 < c {
            println!("No");
            std::process::exit(0);
        }
        dp[cur] = s;
    } else if s == k {
        if c <= 2 {
            dp[cur] = 0;
        } else {
            println!("No");
            std::process::exit(0);
        }
    } else {
        println!("No");
        std::process::exit(0);
    }
}
