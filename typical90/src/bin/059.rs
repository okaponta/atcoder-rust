use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        q:usize,
        mut xy:[(Usize1,Usize1);m],
        ab:[(Usize1,Usize1);q],
    }
    xy.sort();
    // abのクエリを一括で処理する
    for i in (0..q).step_by(128) {
        let mut dp = vec![0u128; n];
        // aにフラグをつける
        for j in 0..128.min(q - i) {
            dp[ab[i + j].0] |= 1 << j;
        }
        // フラグを伝播
        for &(x, y) in &xy {
            dp[y] |= dp[x];
        }
        for j in 0..128.min(q - i) {
            let ans = dp[ab[i + j].1] >> j & 1 == 1;
            println!("{}", if ans { "Yes" } else { "No" });
        }
    }
}
