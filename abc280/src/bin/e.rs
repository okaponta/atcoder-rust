use proconio::input;

fn main() {
    input! {
        n:usize,
        p:i64,
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = 1;
    let unit = modinv(100, 998244353);
    let normal = (100 - p) * unit % 998244353;
    let critical = p * unit % 998244353;
    for i in 2..=n {
        dp[i] = normal * (dp[i - 1] + 1);
        dp[i] %= 998244353;
        dp[i] += critical * (dp[i - 2] + 1);
        dp[i] %= 998244353;
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
