use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:usize,
    }
    let modm = modinv(m as i64, MOD as i64) as usize;
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for _ in 0..k {
        let mut ndp = vec![0; n + 1];
        // 今のマス
        for i in 0..n {
            // でる目
            for j in 1..=m {
                let mut next = i + j;
                if n < next {
                    next = 2 * n - next;
                }
                ndp[next] += dp[i] * modm;
                ndp[next] %= MOD;
            }
        }
        ndp[n] += dp[n];
        ndp[n] %= MOD;
        dp = ndp;
    }
    println!("{}", dp[n]);
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
