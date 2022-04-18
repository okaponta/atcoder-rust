use proconio::input;

const INF: i64 = 2_000_000_000_000_000_000;

fn main() {
    input! {
        n:usize,
        x: i64,
        y: i64,
        a: [i64; n],
        b: [i64; n],
    }
    // dp[S] bitが立っているやつは使ったやつ(頭から決めていく)
    let mut dp = vec![INF; 1 << n];
    dp[0] = 0;
    for bit in 0usize..(1 << n) {
        // 既に決めた数字たち
        let cnt = bit.count_ones() as usize;
        for p in 0..n {
            if (bit >> p) & 1 == 1 {
                // 既に使っているので操作不要
                continue;
            }
            let next = dp[bit]
                + (a[p] - b[cnt]).abs() * x
                + (bit & ((1_usize << n) - (1_usize << p))).count_ones() as i64 * y;
            dp[bit | 1 << p] = dp[bit | 1 << p].min(next);
        }
    }

    println!("{}", dp[(1 << n) - 1]);
}
