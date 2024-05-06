use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n],
    }
    let mut q = VecDeque::new();
    let mut g = vec![vec![]; n];
    for i in 0..n {
        let (a, b) = ab[i];
        if a == i || b == i {
            q.push_back(i);
            continue;
        }
        g[a].push(i);
        g[b].push(i);
    }
    let mut used = vec![false; n];
    let mut ans = vec![];
    while let Some(i) = q.pop_front() {
        if used[i] {
            continue;
        }
        used[i] = true;
        ans.push(i);
        for &next in &g[i] {
            if !used[next] {
                q.push_back(next);
            }
        }
    }
    if ans.len() != n {
        println!("{}", -1);
        return;
    }
    for &i in ans.iter().rev() {
        println!("{}", i + 1);
    }
}
