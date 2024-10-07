#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let ans = dfs(0, 0, &g);
    println!("{}", (ans.0 + ans.1) % MOD);
}

fn dfs(cur: usize, prev: usize, g: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut tw = 1;
    let mut tb = 1;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        let (w, b) = dfs(next, cur, g);
        tw = tw * (w + b) % MOD;
        tb = tb * w % MOD;
    }
    (tw, tb)
}
