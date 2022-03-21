use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        h:usize,w:usize,
        c:usize,
        mut a:[[usize;w];h],
    }
    let ans = solve(h, w, c, &a);
    // 左下から右上にいく経路
    a.reverse();
    println!("{}", ans.min(solve(h, w, c, &a)));
}

fn solve(h: usize, w: usize, c: usize, a: &Vec<Vec<usize>>) -> usize {
    let mut dp = vec![vec![1 << 60; w + 1]; h + 1];
    let mut ans = 1 << 60;
    for (i, j) in iproduct!(0..h, 0..w) {
        dp[i + 1][j + 1] = a[i][j].min(dp[i][j + 1] + c).min(dp[i + 1][j] + c);
        ans = ans.min(c + a[i][j] + dp[i][j + 1].min(dp[i + 1][j]));
    }
    ans
}
