use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        uvw:[(Usize1,Usize1,usize);m],
        k:usize,
        a:[Usize1;k],
        d:usize,
        x:[usize;d],
    }
    let mut g = vec![vec![]; n];
    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut ans = vec![!0; n];
    let mut heap = BinaryHeap::new();
    for &a in &a {
        ans[a] = 0;
        for &(u, w) in &g[a] {
            heap.push((Reverse(w), u));
        }
    }
    let mut used = vec![false; n];
    for i in 0..d {
        let mut infected = BinaryHeap::new();
        while let Some((Reverse(w), v)) = heap.pop() {
            if x[i] < w {
                heap.push((Reverse(w), v));
                break;
            }
            if ans[v] == !0 {
                ans[v] = i + 1;
                infected.push((x[i] - w, v));
            }
        }
        while let Some((s, v)) = infected.pop() {
            if used[v] {
                continue;
            }
            used[v] = true;
            for &(u, w) in &g[v] {
                if ans[u] == !0 || ans[u] == i + 1 {
                    if w <= s {
                        ans[u] = i + 1;
                        infected.push((s - w, u));
                    } else {
                        heap.push((Reverse(w), u));
                    }
                }
            }
        }
    }
    for ans in ans {
        println!("{}", ans as i64);
    }
}
