use proconio::{input, marker::Chars};

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    // dp[j][k]=これまで使用した数字の集合(j)と直前の数字(k)の数
    let mut dp = vec![vec![0i64; 11]; 2 << 10];
    dp[0][10] = 1;
    for i in 0..n {
        let mut next = dp.clone();
        let cur = (s[i] as u8 - b'A') as usize;
        for j in 0..2 << 10 {
            for k in 0..11 {
                // kが同じ場合はそのまま2倍
                if k == cur {
                    next[j][k] += dp[j][k];
                    next[j][k] %= MOD;
                } else {
                    // kが違う場合
                    let cur_bit = 1 << cur;
                    if j >> cur & 1 == 1 {
                        // 1) jに含む場合は飛ばす
                        continue;
                    } else {
                        // 2) jに含まない場合はkのビットを追加して直前をkにする
                        next[j | cur_bit][cur] += dp[j][k];
                        next[j | cur_bit][cur] %= MOD;
                    }
                }
            }
        }
        dp = next;
    }
    let mut ans = 0;
    for j in 0..2 << 10 {
        for k in 0..10 {
            ans += dp[j][k];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
