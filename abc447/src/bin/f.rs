#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        q:usize,
    }
    for _ in 0..q {
        case();
    }
}

fn case() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dp = vec![0; n];
    let mut ans = 1;
    dfs(0, 0, &g, &mut dp, &mut ans);
    println!("{}", ans);
}

fn dfs(cur: usize, prev: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<usize>, ans: &mut usize) {
    let mut children = vec![];
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, g, dp, ans);
        children.push(dp[next]);
    }
    children.sort();
    children.reverse();
    let len = g[cur].len();
    if 4 <= len {
        dp[cur] = children[0] + 1;
        *ans = (*ans).max(children[0] + children[1] + 1);
    } else if len == 3 {
        dp[cur] = 1;
        *ans = (*ans).max(children[0] + 1);
    }
}
