use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        xy:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    let mut set = HashSet::new();
    for (x, y) in xy {
        if set.contains(&(x, y)) {
            continue;
        }
        set.insert((x, y));
        g[x].push(y);
    }
    let topo = topo(n, &g);
    if topo.len() != n {
        println!("No");
        return;
    }
    for i in 1..n {
        if !set.contains(&(topo[i - 1], topo[i])) {
            println!("No");
            return;
        }
    }
    println!("Yes");
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[topo[i]] = i;
    }
    println!("{}", ans.into_iter().map(|e| e + 1).join(" "));
}

fn topo(n: usize, g: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut ind = vec![0; n];
    for i in 0..n {
        for j in 0..g[i].len() {
            ind[g[i][j]] += 1;
        }
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if ind[i] == 0 {
            q.push_back(i);
        }
    }
    let mut topo = vec![];
    while let Some(cur) = q.pop_front() {
        topo.push(cur);
        for &next in g[cur].iter() {
            ind[next] -= 1;
            if ind[next] == 0 {
                q.push_back(next);
            }
        }
    }
    topo
}
