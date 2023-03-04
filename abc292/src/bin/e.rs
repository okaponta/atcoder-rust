use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
    }
    let mut count = 0;
    for i in 0..n {
        let mut used = vec![false; n];
        count += dfs(i, &g, &mut used) - 1;
    }
    println!("{}", count - m);
}

fn dfs(cur: usize, edges: &Vec<Vec<usize>>, used: &mut Vec<bool>) -> usize {
    let mut res = 1;
    used[cur] = true;
    for &next in &edges[cur] {
        if used[next] {
            continue;
        }
        res += dfs(next, edges, used);
    }
    res
}
