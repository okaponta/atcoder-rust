use proconio::input;

fn main() {
    input! {
        n:usize,
        xy:[(u8,i64);n],
    }
    let mut dp = vec![0, -1 << 60];
    for (x, y) in xy {
        let mut next = vec![-1 << 60; 2];
        if x == 0 {
            next[0] = dp[0].max(dp[0] + y).max(dp[1] + y);
            next[1] = dp[1];
        } else {
            next[0] = dp[0];
            next[1] = dp[1].max(dp[0] + y);
        }
        dp = next;
    }
    println!("{}", dp[0].max(dp[1]));
}
