use std::collections::BinaryHeap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,q:usize,
        x:[usize;n],
        ab:[(Usize1,Usize1);n-1],
        vk:[(Usize1,Usize1);q],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in ab {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut dp = vec![vec![0; 21]; n];
    dfs(&mut dp, 0, 0, &x, &edges);
    for (v, k) in vk {
        println!("{}", dp[v][k]);
    }
}

fn dfs(
    dp: &mut Vec<Vec<usize>>,
    cur: usize,
    prev: usize,
    num: &Vec<usize>,
    edges: &Vec<Vec<usize>>,
) {
    let mut heap = BinaryHeap::new();
    heap.push(num[cur]);
    for &next in edges[cur].iter() {
        if next == prev {
            continue;
        }
        dfs(dp, next, cur, num, edges);
        for i in &dp[next] {
            heap.push(*i);
        }
    }
    let mut count = 0;
    for _ in 0..21 {
        let i = heap.pop();
        if i == None {
            break;
        }
        dp[cur][count] = i.unwrap();
        count += 1;
    }
    heap.clear();
}
