use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        uv:[(Usize1,Usize1);n-1]
    }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    if g[0].len() == 1 {
        println!("1");
        return;
    }
    let mut ans = 0;
    for &next in &g[0] {
        ans = ans.max(dfs(next, 0, &g));
    }
    println!("{}", n - ans);
}

fn dfs(cur: usize, prev: usize, g: &Vec<Vec<usize>>) -> usize {
    let mut res = 1;
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        res += dfs(next, cur, g);
    }
    res
}
