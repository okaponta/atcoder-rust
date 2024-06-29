use proconio::input;

fn main() {
    input! {
        n:usize,
        a:usize,
        x:[usize;n],
    }
    let mut dp = vec![vec![0usize; 2501]; n + 1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in (0..=i).rev() {
            for k in 0..=j * 50 {
                dp[j + 1][k + x[i]] += dp[j][k];
            }
        }
    }
    let mut ans = 0;
    for i in 1..=n {
        ans += dp[i][i * a];
    }
    println!("{}", ans);
}
