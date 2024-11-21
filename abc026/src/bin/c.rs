#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        b:[Usize1;n-1],
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[b[i]].push(i + 1);
    }
    let ans = dfs(0, &g);
    println!("{}", ans);
}

fn dfs(cur: usize, g: &Vec<Vec<usize>>) -> usize {
    if g[cur].len() == 0 {
        return 1;
    }
    let mut min = 1 << 60;
    let mut max = 0;
    for &next in &g[cur] {
        let n = dfs(next, g);
        min = min.min(n);
        max = max.max(n);
    }
    min + max + 1
}
