use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    if n % 2 == 0 {
        even(n, a);
    } else {
        odd(n, a);
    }
}

fn even(n: usize, a: Vec<i64>) {
    let mut dp = vec![vec![-1 << 60; 2]; n];
    for i in 0..2 {
        dp[i][i] = a[i];
    }
    for i in 2..n {
        for j in 0..2 {
            dp[i][j] = dp[i][j].max(dp[i - 2][j] + a[i]);
        }
        if 2 < i {
            dp[i][1] = dp[i][1].max(dp[i - 3][0] + a[i]);
        }
    }
    println!("{}", dp[n - 1][1].max(dp[n - 2][0]));
}

fn odd(n: usize, a: Vec<i64>) {
    let mut dp = vec![vec![-1 << 60; 3]; n];
    for i in 0..3 {
        dp[i][i] = a[i];
    }
    dp[2][0] = dp[0][0] + a[2];
    for i in 3..n {
        for j in 0..3 {
            dp[i][j] = dp[i][j].max(dp[i - 2][j] + a[i]);
        }
        for j in 0..2 {
            dp[i][j + 1] = dp[i][j + 1].max(dp[i - 3][j] + a[i]);
        }
        if 3 < i {
            dp[i][2] = dp[i][2].max(dp[i - 4][0] + a[i]);
        }
    }
    println!("{}", dp[n - 1][2].max(dp[n - 2][1]).max(dp[n - 3][0]));
}
