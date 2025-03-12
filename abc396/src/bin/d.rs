use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        m:usize,
        uvw:[(Usize1,Usize1,usize);m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    let mut used = vec![false; n];
    used[0] = true;
    println!("{}", dfs(0, 0, n, &g, &mut used));
}

fn dfs(
    cur: usize,
    tmp: usize,
    n: usize,
    g: &Vec<Vec<(usize, usize)>>,
    used: &mut Vec<bool>,
) -> usize {
    if cur == n - 1 {
        return tmp;
    }
    let mut res = 1 << 62;
    for &(next, w) in &g[cur] {
        if used[next] {
            continue;
        }
        used[next] = true;
        res = res.min(dfs(next, tmp ^ w, n, g, used));
        used[next] = false;
    }
    res
}
