use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        testcase();
    }
}

fn testcase() {
    input! {
        n:usize,
        m:usize,
        c:[u8;n],
        uv:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut used = vec![false; n * n];
    used[n - 1] = true;
    let mut q = VecDeque::new();
    q.push_back((0, n - 1, 0));
    while let Some((a, b, cost)) = q.pop_front() {
        for &na in &g[a] {
            for &nb in &g[b] {
                if used[na * n + nb] || c[na] == c[nb] {
                    continue;
                }
                if na == n - 1 && nb == 0 {
                    println!("{}", cost + 1);
                    return;
                } else {
                    q.push_back((na, nb, cost + 1));
                    used[na * n + nb] = true;
                }
            }
        }
    }
    println!("-1");
}
