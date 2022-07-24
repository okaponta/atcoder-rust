use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        x:[usize;n],
        cy:[(usize,usize);m],
    }
    let mut bonus = vec![0; n + 1];
    for (c, y) in cy {
        bonus[c] = y;
    }
    // dp[i] i回連続のうち最大のカウント
    let mut dp = vec![0; n + 1];
    for i in 0..n {
        let mut next = vec![0; n + 1];
        for j in 0..=i {
            // 表
            next[j + 1] = dp[j] + x[i] + bonus[j + 1];
            // 裏
            next[0] = next[0].max(dp[j]);
        }
        dp = next;
    }
    println!("{}", dp.iter().max().unwrap());
}
