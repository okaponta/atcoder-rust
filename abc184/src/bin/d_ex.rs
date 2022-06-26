use proconio::input;

fn main() {
    let mut dp = vec![vec![vec![0.0; 101]; 101]; 101];
    for i in (0..100).rev() {
        for j in (0..100).rev() {
            for k in (0..100).rev() {
                let sum = (i + j + k) as f64;
                dp[i][j][k] = (i as f64) / sum * dp[i + 1][j][k]
                    + (j as f64) / sum * dp[i][j + 1][k]
                    + (k as f64) / sum * dp[i][j][k + 1]
                    + 1.0;
            }
        }
    }
    input!(a: usize, b: usize, c: usize);
    println!("{}", dp[a][b][c]);
}
