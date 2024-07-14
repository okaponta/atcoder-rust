use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[usize;n],
        xy:[(Usize1,Usize1);q],
    }
    let mut ng = vec![HashSet::new(); n];
    for (x, y) in xy {
        ng[y].insert(x);
    }
    dfs(0, vec![], n, &a, &ng, &mut vec![vec![]; 9000]);
}

fn dfs(
    cur: usize,
    selected: Vec<usize>,
    n: usize,
    a: &Vec<usize>,
    ng: &Vec<HashSet<usize>>,
    ok: &mut Vec<Vec<usize>>,
) {
    if cur == n {
        let s = selected.iter().map(|&i| a[i]).sum::<usize>();
        if ok[s].is_empty() {
            ok[s] = selected;
        } else {
            print(ok[s].clone());
            print(selected);
            std::process::exit(0);
        }
        return;
    }
    if selected.iter().all(|i| !ng[cur].contains(i)) {
        let mut sc = selected.clone();
        sc.push(cur);
        dfs(cur + 1, sc, n, a, ng, ok);
    }
    dfs(cur + 1, selected, n, a, ng, ok);
}

fn print(a: Vec<usize>) {
    println!("{}", a.len());
    println!("{}", a.iter().map(|i| i + 1).join(" "));
}
