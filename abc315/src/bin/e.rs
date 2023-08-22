use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
    }
    let mut g = vec![vec![]; n];
    for i in 0..n {
        input! {
            c:usize,
            p:[Usize1;c],
        }
        for p in p {
            g[i].push(p);
        }
    }
    let mut visited = vec![false; n];
    let mut ans = vec![];
    dfs(0, &g, &mut ans, &mut visited);
    ans.pop();
    println!("{}", ans.into_iter().join(" "));
}

fn dfs(cur: usize, g: &Vec<Vec<usize>>, ans: &mut Vec<usize>, visited: &mut Vec<bool>) {
    if visited[cur] {
        return;
    }
    visited[cur] = true;
    for &next in &g[cur] {
        dfs(next, g, ans, visited);
    }
    ans.push(cur + 1);
}
