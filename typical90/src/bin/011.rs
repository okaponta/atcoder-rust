use proconio::input;

fn main() {
    input! {
        n:usize,
        mut dcs:[(usize,usize,i64);n],
    }
    dcs.sort();
    let m = dcs[n - 1].0;
    // dp[i][j] i番目の仕事までで、j日目の仕事を見た場合の最大
    let mut dp = vec![-1; m + 1];
    dp[0] = 0;
    for i in 0..n {
        let (d, c, s) = dcs[i];
        for j in (0..m).rev() {
            if dp[j] == -1 || d < j + c {
                continue;
            }
            dp[j + c] = dp[j + c].max(dp[j] + s);
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
