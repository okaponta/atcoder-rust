use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let mut ans = vec![1i64 << 60; n + m];
    let mut used = vec![false; n + m];
    let mut g = vec![vec![]; n + m];
    for i in 0..n {
        input! {
            a:usize,
            s:[Usize1;a]
        }
        for j in s {
            g[i].push(n + j);
            g[n + j].push(i);
        }
    }
    let mut q = VecDeque::new();
    q.push_back((n, 0));
    while let Some((i, j)) = q.pop_front() {
        if used[i] {
            continue;
        }
        used[i] = true;
        ans[i] = j;
        for &next in &g[i] {
            if used[next] {
                continue;
            }
            q.push_back((next, j + 1));
        }
    }
    println!(
        "{}",
        if ans[n + m - 1] == 1 << 60 {
            -1
        } else {
            (ans[n + m - 1] - 2) / 2
        }
    );
}
