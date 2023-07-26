use std::collections::VecDeque;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
        s:Usize1,
        t:Usize1,
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
    }
    let mut dist = vec![vec![1usize << 60; 3]; n];
    let mut q = VecDeque::new();
    q.push_back((s, 0, 0));
    while let Some((cur, d, turn)) = q.pop_front() {
        if dist[cur][turn] <= d {
            continue;
        }
        dist[cur][turn] = d;
        for &next in &g[cur] {
            let nextd = if (turn + 1) % 3 == 0 { d + 1 } else { d };
            if nextd < dist[next][(turn + 1) % 3] {
                q.push_back((next, nextd, (turn + 1) % 3));
            }
        }
    }
    let ans = dist[t][0];
    println!("{}", if ans == 1 << 60 { -1 } else { ans as i64 });
}
