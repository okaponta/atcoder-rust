#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n1:usize,
        uv1:[(Usize1,Usize1);n1-1],
        n2:usize,
        uv2:[(Usize1,Usize1);n2-1],
    }
    let mut g1 = vec![vec![]; n1];
    let mut g2 = vec![vec![]; n2];
    for (u, v) in uv1 {
        g1[u].push(v);
        g1[v].push(u);
    }
    for (u, v) in uv2 {
        g2[u].push(v);
        g2[v].push(u);
    }
    let (mut d1, di1) = tree_max_len(&g1);
    let (mut d2, di2) = tree_max_len(&g2);
    d1.sort();
    d2.sort();
    d2.reverse();
    let mut ans = 0;
    let mut sum = 0;
    let mut rem = n2;
    let mut tmp = 0;
    for i in 0..n1 {
        while tmp < n2 && di1.max(di2) < 1 + d1[i] + d2[tmp] {
            sum += d2[tmp];
            tmp += 1;
            rem -= 1;
        }
        ans += rem * di1.max(di2) + tmp * (d1[i] + 1) + sum;
    }
    println!("{}", ans);
}

fn tree_diameter(edges: &Vec<Vec<usize>>) -> (usize, usize, usize) {
    let l = tree_diameter_dfs(edges, 0, !0);
    let r = tree_diameter_dfs(edges, l.1, !0);
    (l.1, r.1, r.0)
}

// (距離, to)の最大値を返却する
fn tree_diameter_dfs(edges: &Vec<Vec<usize>>, cur: usize, parent: usize) -> (usize, usize) {
    let mut ret = (0, cur);
    for &to in &edges[cur] {
        if to == parent {
            continue;
        }
        let mut next = tree_diameter_dfs(edges, to, cur);
        next.0 += 1;
        ret = ret.max(next);
    }
    ret
}

// それぞれの辺から木の中での最長距離を求める
fn tree_max_len(edges: &Vec<Vec<usize>>) -> (Vec<usize>, usize) {
    let n = edges.len();
    let (l, r, d) = tree_diameter(edges);
    let mut res = vec![0; n];
    let mut d1 = vec![0; n];
    let mut d2 = vec![0; n];
    dfs2(l, l, &edges, &mut d1);
    dfs2(r, r, &edges, &mut d2);
    for i in 0..n {
        res[i] = d1[i].max(d2[i]);
    }
    (res, d)
}

fn dfs2(prev: usize, cur: usize, edges: &Vec<Vec<usize>>, d: &mut Vec<usize>) {
    for &next in &edges[cur] {
        if next == prev {
            continue;
        }
        d[next] = d[cur] + 1;
        dfs2(cur, next, edges, d);
    }
}
