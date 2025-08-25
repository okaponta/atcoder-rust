use proconio::{marker::*, *};

fn main() {
    input! {n:usize, mut x:[i64;n], uvw:[(Usize1,Usize1,usize);n-1]}
    let mut g = vec![vec![]; n];
    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    println!("{}", dfs(0, 0, &mut x, &g));
}

fn dfs(cur: usize, prev: usize, x: &mut Vec<i64>, g: &Vec<Vec<(usize, usize)>>) -> usize {
    let mut res = 0;
    for &(next, cost) in &g[cur] {
        if next == prev {
            continue;
        }
        res += dfs(next, cur, x, g);
        x[cur] += x[next];
        res += (x[next].abs() as usize) * cost;
    }
    res
}
