use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
    }
    if m == 1 {
        println!("1");
        return;
    }
    if k == 0 {
        println!("{}", pow(m as i64, n as i64, MOD as i64));
        return;
    }
    // 差を管理します
    let mut dp = vec![0; m];
    dp[0] = 1;
    for _ in 1..n {
        let mut next = vec![0; m];
        let mut temp = 0;
        for i in 0..m {
            temp = (temp + dp[i]) % MOD;
            if i < m - k {
                // 右が対象
                next[i + k] += temp;
                next[i + k] %= MOD;
            }
            if k <= i {
                // 左が対象
                next[0] += temp;
                next[0] %= MOD;
                next[i + 1 - k] = (next[i + 1 - k] + MOD - temp) % MOD;
            }
        }
        dp = next;
    }
    let mut ans = 0;
    let mut tmp = 0;
    for i in 0..m {
        tmp += dp[i];
        tmp %= MOD;
        ans += tmp;
        ans %= MOD;
    }
    println!("{}", ans);
}

fn pow(mut x: i64, mut n: i64, modulo: i64) -> i64 {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
