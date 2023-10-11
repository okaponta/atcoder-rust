use std::process::exit;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        lrd:[(Usize1,Usize1,i64);m],
    }
    let mut g = vec![vec![]; n];
    for (l, r, d) in lrd {
        g[l].push((r, d));
        g[r].push((l, -d));
    }
    let mut used = vec![false; n];
    let mut ans = vec![0; n];
    for i in 0..n {
        if used[i] {
            continue;
        }
        used[i] = true;
        dfs(i, i, &mut used, &g, &mut ans);
    }
    println!("Yes");
}

fn dfs(
    cur: usize,
    prev: usize,
    used: &mut Vec<bool>,
    g: &Vec<Vec<(usize, i64)>>,
    ans: &mut Vec<i64>,
) {
    for &(next, cost) in &g[cur] {
        if used[next] || next == prev {
            if ans[cur] + cost != ans[next] {
                println!("No");
                exit(0);
            }
        } else {
            ans[next] = ans[cur] + cost;
            used[next] = true;
            dfs(next, cur, used, g, ans);
        }
    }
}
