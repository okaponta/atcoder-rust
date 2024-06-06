use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        ab:[(usize,usize);n],
    }
    let m = 5000;
    let mut c = vec![vec![0; m + 1]; m + 1];
    for (a, b) in ab {
        c[a][b] += 1;
    }
    let mut dp = vec![vec![0; m + 1]; m + 1];
    let mut ans = vec![vec![0; m + 1]; m + 1];
    let mut used = vec![vec![false; m + 1]; m + 1];
    dfs(m, m, k, &c, &mut dp, &mut used, &mut ans);
    println!("{}", ans.iter().flatten().max().unwrap());
}

fn dfs(
    a: usize,
    b: usize,
    k: usize,
    c: &Vec<Vec<usize>>,
    dp: &mut Vec<Vec<usize>>,
    used: &mut Vec<Vec<bool>>,
    ans: &mut Vec<Vec<usize>>,
) -> usize {
    if used[a][b] || a == 0 || b == 0 {
        return dp[a][b];
    }
    used[a][b] = true;
    let res = dfs(a - 1, b, k, c, dp, used, ans) + dfs(a, b - 1, k, c, dp, used, ans)
        - dfs(a - 1, b - 1, k, c, dp, used, ans)
        + c[a][b];
    dp[a][b] = res;
    if k < a && k < b {
        ans[a][b] = dp[a][b] + dp[a - k - 1][b - k - 1] - dp[a - k - 1][b] - dp[a][b - k - 1];
    }
    res
}
