use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
        q:usize,
        tex:[(u8,Usize1,i64);q],
    }
    let mut edges = vec![vec![]; n];
    let mut map = HashMap::new();
    for (t, e, x) in tex {
        if t == 1 {
            *map.entry((ab[e].0, ab[e].1)).or_insert(0) += x;
        } else {
            *map.entry((ab[e].1, ab[e].0)).or_insert(0) += x;
        }
    }
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut ans = vec![0; n];
    dfs1(0, 0, &edges, &map, &mut ans);
    dfs2(0, 0, &edges, &mut ans);
    for i in ans {
        println!("{}", i);
    }
}

fn dfs1(
    cur: usize,
    prev: usize,
    edges: &Vec<Vec<usize>>,
    map: &HashMap<(usize, usize), i64>,
    ans: &mut Vec<i64>,
) {
    if let Some(val) = map.get(&(prev, cur)) {
        ans[0] += val;
        ans[cur] -= val;
    }
    if let Some(val) = map.get(&(cur, prev)) {
        ans[cur] += val;
    }
    for &next in &edges[cur] {
        if prev == next {
            continue;
        }
        dfs1(next, cur, edges, map, ans);
    }
}

fn dfs2(cur: usize, prev: usize, edges: &Vec<Vec<usize>>, ans: &mut Vec<i64>) {
    if cur != prev {
        ans[cur] += ans[prev];
    }
    for &next in &edges[cur] {
        if prev == next {
            continue;
        }
        dfs2(next, cur, edges, ans);
    }
}
