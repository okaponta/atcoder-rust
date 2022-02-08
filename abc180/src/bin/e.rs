use itertools::iproduct;
use proconio::input;

const INF: i64 = 1 << 60;

fn main() {
    input! {
       n:usize,
       xyz:[(i64,i64,i64);n],
    }
    // 先に各点への距離を計算しておくと多少はやくなる
    let mut dist = vec![vec![0; n]; n];
    for (i, j) in iproduct!(0..n, 0..n) {
        dist[i][j] =
            (xyz[i].0 - xyz[j].0).abs() + (xyz[i].1 - xyz[j].1).abs() + 0.max(xyz[j].2 - xyz[i].2);
    }
    // dp[i][j] i:通った点, j:直前の点
    let mut dp = vec![vec![INF; n]; 1 << n];
    dp[0][0] = 0;
    // ちょっと直感的じゃないけど、0..1<<n でループすれば答えは求まる
    for (i, j, k) in iproduct!(0..1 << n, 0..n, 0..n) {
        // 行き先のフラグが立ってないもの(訪問していない都市に限って処理)
        // もとの都市にフラグが立っているので絞っても実行時間変わらずなので、これでよさそう
        if i >> k & 1 == 0 {
            let d = dp[i][j] + dist[j][k];
            dp[i | 1 << k][k] = dp[i | 1 << k][k].min(d);
        }
    }
    println!("{}", dp[(1 << n) - 1][0]);
}
