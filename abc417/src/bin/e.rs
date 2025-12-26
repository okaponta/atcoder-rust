#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        t:usize,
    }
    for _ in 0..t {
        case();
    }
}

fn case() {
    input! {
        n:usize,
        m:usize,
        x:Usize1,
        y:Usize1,
        uv:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    for i in 0..n {
        g[i].sort();
    }
    let mut ans = vec![x];
    let mut used = HashSet::new();
    used.insert(x);
    dfs(&mut ans, &mut used, x, y, n, &g);
    println!("{}", ans.iter().map(|i| i + 1).join(" "));
}

fn dfs(
    ans: &mut Vec<usize>,
    used: &mut HashSet<usize>,
    cur: usize,
    y: usize,
    n: usize,
    g: &Vec<Vec<usize>>,
) -> bool {
    for &next in &g[cur] {
        if used.contains(&next) {
            continue;
        } else {
            if next == y {
                ans.push(next);
                used.insert(next);
                return true;
            }
            used.insert(next);
            ans.push(next);
            let v = dfs(ans, used, next, y, n, g);
            if !v {
                // used.remove(&next);
                ans.pop();
                continue;
            }
            return v;
        }
    }
    false
}
