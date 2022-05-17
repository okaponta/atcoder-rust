use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
        n:usize,m:usize,k:usize,
        w:[i64;n],
    }
    let wsum = w.iter().sum::<i64>();
    let wsuminv = modinv(wsum, MOD);
    let mut fact = vec![1];
    for i in 0..50 {
        fact.push(fact[i as usize] * (i + 1) % MOD);
    }
    let mut factinv = vec![];
    for f in fact.iter() {
        factinv.push(modinv(*f, MOD));
    }
    let mut p = vec![];
    for wi in w {
        p.push(wi * wsuminv % MOD);
    }
    let mut dp = vec![vec![vec![0; k + 1]; m + 2]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        for j in 0..=m {
            for l in 0..=k {
                for m in 0..=k - l {
                    if m == 0 {
                        dp[i + 1][j][l] += dp[i][j][l];
                        dp[i + 1][j][l] %= MOD;
                    } else {
                        dp[i + 1][j + 1][l + m] +=
                            dp[i][j][l] * factinv[m] % MOD * pow(p[i], m as i64, MOD);
                        dp[i + 1][j + 1][l + m] %= MOD;
                    }
                }
            }
        }
    }
    println!("{}", dp[n][m][k] * fact[k] % MOD);
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
