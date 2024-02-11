use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut set = HashSet::new();
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    dfs(0, 0, &g, &mut set, &mut set1, &mut set2);
    let mut ans = vec![];
    if n / 2 <= set1.len() {
        for i in set1 {
            ans.push(i + 1);
        }
    } else {
        for i in set2 {
            ans.push(i + 1);
        }
    }
    while ans.len() != n / 2 {
        ans.pop();
    }
    println!("{}", ans.iter().join(" "));
}

fn dfs(
    cur: usize,
    prev: usize,
    g: &Vec<Vec<usize>>,
    set: &mut HashSet<usize>,
    set1: &mut HashSet<usize>,
    set2: &mut HashSet<usize>,
) {
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        if set.contains(&next) {
            continue;
        }
        if set1.contains(&cur) {
            set2.insert(next);
        } else {
            set1.insert(next);
        }
        dfs(next, cur, g, set, set1, set2);
    }
}
