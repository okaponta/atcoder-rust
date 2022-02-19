use proconio::input;

const MOD: i64 = 998244353;

fn main() {
    input! {
       n:i64, k:i64, m:i64,
    }
    let r = pow(k, n, MOD - 1);
    println!("{}", pow(m, r, MOD));
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
