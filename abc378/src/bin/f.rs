use std::collections::VecDeque;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n-1],
    }
    let mut jisuu = vec![0; n];
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        jisuu[u] += 1;
        jisuu[v] += 1;
        g[u].push(v);
        g[v].push(u);
    }
    let mut ans = 0;
    let mut used = vec![false; n];
    let mut q = VecDeque::new();
    for i in 0..n {
        if jisuu[i] == 3 && !used[i] {
            used[i] = true;
            q.push_back(i);
        }
        let mut num = 0usize;
        while let Some(j) = q.pop_front() {
            for &next in &g[j] {
                if jisuu[next] == 2 {
                    num += 1;
                }
                if jisuu[next] == 3 && !used[next] {
                    used[next] = true;
                    q.push_back(next);
                }
            }
        }
        if num < 2 {
            continue;
        }
        ans += num * (num - 1) / 2;
    }
    println!("{}", ans);
}
