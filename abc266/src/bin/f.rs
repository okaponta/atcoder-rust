use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n],
        q:usize,
        xy:[(Usize1,Usize1);q],
    }
    let mut e = vec![vec![]; n];
    for (u, v) in uv {
        e[u].push(v);
        e[v].push(u);
    }
    let mut is_cycle = vec![true; n];
    let mut deg = vec![0; n];
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        heap.push(Reverse((e[i].len(), i)));
        deg[i] = e[i].len();
    }
    while let Some(Reverse((len, i))) = heap.pop() {
        if len == 1 {
            is_cycle[i] = false;
            for &next in &e[i] {
                if is_cycle[next] {
                    deg[next] -= 1;
                    heap.push(Reverse((deg[next], next)));
                }
            }
        } else {
            break;
        }
    }
    let mut root = vec![!0; n];
    for i in 0..n {
        if is_cycle[i] {
            dfs(i, i, i, &e, &is_cycle, &mut root);
        }
    }
    for (x, y) in xy {
        println!("{}", if root[x] == root[y] { "Yes" } else { "No" });
    }
}

fn dfs(
    cur: usize,
    prev: usize,
    r: usize,
    e: &Vec<Vec<usize>>,
    is_cycle: &Vec<bool>,
    root: &mut Vec<usize>,
) {
    root[cur] = r;
    for &next in &e[cur] {
        if next == prev || is_cycle[next] {
            continue;
        }
        dfs(next, cur, r, e, is_cycle, root);
    }
}
