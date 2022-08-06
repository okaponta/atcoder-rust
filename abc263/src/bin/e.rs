use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,
        a:[i64;n-1],
    }
    let mut sum = vec![0; n + 1];
    let mut dp = vec![0; n];
    for i in (0..n - 1).rev() {
        let inv = modinv(a[i], MOD);
        let s = (MOD + sum[i + 1] - sum[i + 1 + a[i] as usize]) % MOD;
        dp[i] = (1 + inv + inv * s) % MOD;
        sum[i] = (sum[i + 1] + dp[i]) % MOD;
    }
    println!("{}", dp[0]);
}

fn modinv(mut a: i64, modulo: i64) -> i64 {
    let mut b = modulo;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= modulo;
    if u < 0 {
        u += modulo;
    }
    u
}
