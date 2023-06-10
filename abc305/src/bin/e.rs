use std::collections::{BTreeSet, BinaryHeap};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
        ab:[(Usize1,Usize1);m],
        mut ph:[(Usize1,usize);k],
    }
    let mut ans = BTreeSet::new();
    let mut maxh = vec![0; n];
    let mut heap = BinaryHeap::new();
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    ph.sort_by_key(|k| k.1);
    ph.reverse();
    for (p, h) in ph {
        heap.push((h, p));
        maxh[p] = h;
        ans.insert(p + 1);
    }
    while let Some((h, p)) = heap.pop() {
        if h < maxh[p] {
            continue;
        }
        for &next in &g[p] {
            if 1 < h && h - 1 <= maxh[next] {
                continue;
            }
            if 1 < h {
                heap.push((h - 1, next));
                maxh[next] = h - 1;
            }
            ans.insert(next + 1);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
