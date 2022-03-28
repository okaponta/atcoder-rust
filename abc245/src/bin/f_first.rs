use std::collections::{HashSet, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize, m: usize,
        uv: [(Usize1, Usize1); m],
    }
    let mut edge = vec![HashSet::new(); n];
    let mut edge_rev = vec![HashSet::new(); n];
    for &(u, v) in uv.iter() {
        edge[u].insert(v);
        edge_rev[v].insert(u);
    }
    let mut queue = VecDeque::new();
    for i in 0..n {
        if edge[i].is_empty() {
            queue.push_back(i);
        }
    }

    let mut ans = n;
    while let Some(q) = queue.pop_front() {
        ans -= 1;
        for &p in &edge_rev[q] {
            edge[p].remove(&q);
            if edge[p].is_empty() {
                queue.push_back(p);
            }
        }
    }

    println!("{}", ans);
}
