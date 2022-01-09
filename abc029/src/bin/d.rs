use proconio::{input, marker::Chars};

fn main() {
    input! {
       nchar:Chars,
    }
    let n = nchar.len();
    // dp[i][0][j] i桁目までがnと同じ場合に1がj回出現した回数
    // dp[i][1][j] i桁目までがnより小さい場合に1がj回出現した回数
    // i,jの最大値は最大でもnのため、0-nを格納できるようにVecを作成
    let mut dp = vec![vec![vec![0; n + 1]; 2]; n + 1];

    dp[0][0][0] = 1;

    for i in 0..n {
        for j in 0..n {
            // i+1桁目が1以外のとき、出現回数は変わらないのでdp[i+1][1][j]に加算
            dp[i + 1][1][j] += dp[i][1][j] * 9;
            // i+1桁目が1のとき、出現回数が増えるのでdp[i+1][1][j+1]に加算
            dp[i + 1][1][j + 1] += dp[i][1][j];
            // 以上で、i桁目まででnより小さい場合の処理は終わり
            // ちなみに、dp[0][1][j]==0なので、最初の桁は上の処理は意味ない

            let ni = nchar[i].to_digit(10).unwrap() as i32;

            // 以下はi桁目までnと同じ場合の処理

            // i+1桁目はnより小さい数のとき
            if ni > 1 {
                // i+1桁目が1以外のときは出現回数が増えないので、jに加算
                dp[i + 1][1][j] += dp[i][0][j] * (ni - 1);
                // i+1桁目が1のとき、出現回数が増えるのでj+1に加算
                dp[i + 1][1][j + 1] += dp[i][0][j];
            } else if ni == 1 {
                // i+1桁目はゼロ固定
                dp[i + 1][1][j] += dp[i][0][j];
            }
            // i+1桁目がゼロの場合は上記の処理はスキップされる

            // i+1桁目もnと同じ数のとき
            // dp[i+1][0][hoge]への操作はここでのみ行われる
            if ni == 1 {
                // 1の出現回数が増えるのでj+1
                dp[i + 1][0][j + 1] = dp[i][0][j];
            } else {
                // 1の出現回数は変わらないのでj
                dp[i + 1][0][j] = dp[i][0][j];
            }
        }
    }

    let mut ans = 0;
    for i in 0..=n {
        ans += (dp[n][0][i] + dp[n][1][i]) * i as i32;
    }
    println!("{}", ans);
}
