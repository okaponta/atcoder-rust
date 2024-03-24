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
    let mut dp = vec![1; n];
    dfs(0, 0, &mut dp, &g);
    let ans = (0..n).fold(0, |s, i| s + dp[i] * (n - dp[i]));
    println!("{}", ans);
}

fn dfs(cur: usize, prev: usize, dp: &mut Vec<usize>, g: &Vec<Vec<usize>>) {
    for &next in &g[cur] {
        if prev == next {
            continue;
        }
        dfs(next, cur, dp, g);
        dp[cur] += dp[next];
    }
}
