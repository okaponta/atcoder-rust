use proconio::input;

fn main() {
    input! {
        n:usize,
        c:i64,
        a:[i64;n],
        b:[i64;n],
    }
    let mut dp = vec![1 << 60; 1 << n];
    dp[0] = 0;
    for i in 0usize..(1 << n) {
        let s = i.count_ones() as usize;
        for j in 0..n {
            if i >> j & 1 == 1 {
                continue;
            }
            let mut val = dp[i];
            let mut next = i;
            if j > 0 && next >> (j - 1) & 1 == 0 {
                val += c;
            }
            for k in j..n {
                if next >> k & 1 == 1 {
                    break;
                }
                val += (a[k] - b[s + k - j]).abs();
                next |= 1 << k;
                dp[next] = dp[next].min(if k + 1 < n && next >> (k + 1) & 1 == 0 {
                    val + c
                } else {
                    val
                });
            }
        }
    }
    println!("{}", dp[(1 << n) - 1]);
}
