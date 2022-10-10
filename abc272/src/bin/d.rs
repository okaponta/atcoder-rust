use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use num_integer::Roots;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:i64,
    }
    // 平方数を記録する
    let mut sq = HashSet::new();
    for i in 0..=m.sqrt() {
        sq.insert(i * i);
    }
    let mut pair = vec![];
    for i in 0..=m.sqrt() {
        if sq.contains(&(m - i * i)) {
            pair.push((i, (m - i * i).sqrt()));
        }
    }
    let x = vec![1, 1, -1, -1];
    let y = vec![1, -1, 1, -1];
    let mut ans = vec![vec![-1; n]; n];
    let mut q = VecDeque::new();
    ans[0][0] = 0;
    q.push_back((0, 0, 0));
    while let Some((i, j, c)) = q.pop_front() {
        for &(bdx, bdy) in &pair {
            for k in 0..4 {
                let dx = bdx * x[k];
                let dy = bdy * y[k];
                let nextx = i as i64 + dx;
                let nexty = j as i64 + dy;
                if 0 <= nextx && nextx < n as i64 && 0 <= nexty && nexty < n as i64 {
                    let nextx = nextx as usize;
                    let nexty = nexty as usize;
                    if ans[nextx][nexty] == -1 {
                        ans[nextx][nexty] = c + 1;
                        q.push_back((nextx, nexty, c + 1));
                    }
                }
            }
        }
    }
    for i in 0..n {
        println!("{}", ans[i].iter().join(" "));
    }
}
