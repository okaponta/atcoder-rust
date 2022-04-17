use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        p:usize,
    }
    // dp[i][j][k] i個目までのコの字についてj本選び、状態がkである場合の和
    // 状態0：連結。状態1：非連結
    // 実装上はメモリ節約のため、使い回しする
    let mut dp = vec![vec![1, 0], vec![0, 1]];
    for i in 0..n - 1 {
        // dp[j][0]->dp[j+2][1]の遷移があるため、多めに確保。
        // 実際にはi+4めいっぱいまで数字は入らないが場合分けするのも面倒なので
        let mut next = vec![vec![0; 2]; i + 4];
        for j in 0..i + 2 {
            next[j][0] += dp[j][0] + dp[j][1];
            next[j + 1][0] += dp[j][0] * 3;
            next[j + 2][1] += dp[j][0] * 2;
            next[j + 1][1] += dp[j][1];
        }
        for j in 0..i + 3 {
            next[j][0] %= p;
            next[j][1] %= p;
        }
        dp = next;
    }
    let mut ans = vec![];
    for j in 1..n {
        ans.push(dp[j][0]);
    }
    println!("{}", ans.iter().join(" "));
}
