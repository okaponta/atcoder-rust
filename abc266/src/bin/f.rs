use std::collections::VecDeque;

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
    let mut q = VecDeque::new();
    for i in 0..n {
        deg[i] = e[i].len();
        if deg[i] == 1 {
            q.push_back(i);
        }
    }
    // 次数が1の頂点をキューで処理
    while let Some(i) = q.pop_front() {
        is_cycle[i] = false;
        for &next in &e[i] {
            // 既に削除済なら飛ばす
            if is_cycle[next] {
                deg[next] -= 1;
                if deg[next] == 1 {
                    q.push_back(next);
                }
            }
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
