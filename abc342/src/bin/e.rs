use std::collections::BinaryHeap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        ldkcab:[(i64,i64,i64,i64,Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (l, d, k, c, a, b) in ldkcab {
        g[b].push((a, l, d, k, c));
    }
    const INF: i64 = 1 << 60;
    let mut ans = vec![-1; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        if i == n - 1 {
            heap.push((INF, i));
        }
        heap.push((-1, i));
    }
    while let Some((t, target)) = heap.pop() {
        if t < ans[target] {
            continue;
        }
        ans[target] = t;
        for &(next, l, d, k, c) in &g[target] {
            let tmp = calc(t, l, d, k, c);
            if tmp == -1 {
                continue;
            }
            if ans[next] < tmp {
                ans[next] = tmp;
                heap.push((ans[next], next));
            }
        }
    }
    for i in 0..n - 1 {
        if ans[i] == -1 {
            println!("Unreachable");
        } else {
            println!("{}", ans[i]);
        }
    }
}

fn calc(t: i64, l: i64, d: i64, k: i64, c: i64) -> i64 {
    if t - c <= l {
        return -1;
    }
    ((t - c - l) / d).min(k - 1) * d + l
}
