use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        ab:[(Usize1,Usize1);n-1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dp = vec![0; n];
    let mut ans = vec![0; n];
    dfs(0, 0, &g, &mut dp, &mut ans);
    let ans = *ans.iter().max().unwrap();
    if ans < 5 {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(cur: usize, prev: usize, g: &Vec<Vec<usize>>, dp: &mut Vec<usize>, ans: &mut Vec<usize>) {
    let mut child = vec![];
    for &next in &g[cur] {
        if next == prev {
            continue;
        }
        dfs(next, cur, g, dp, ans);
        child.push(dp[next]);
    }
    child.sort();
    child.reverse();
    if child.len() < 3 {
        dp[cur] = 1;
    } else {
        dp[cur] = 1 + child[0] + child[1] + child[2];
    }
    if 3 < child.len() {
        ans[cur] = 1 + child[0] + child[1] + child[2] + child[3];
    } else if 0 < child.len() {
        ans[cur] = 1 + child[0];
    }
}
