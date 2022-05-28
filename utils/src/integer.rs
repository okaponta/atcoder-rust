use num_integer::Roots;

// 1+2+...+n
fn tousa_sum_one(n: i64) -> i64 {
    n * (n + 1) / 2
}

// 1^2 + 2^2 +...+ n^2
fn tousa_square_sum_one(n: i64) -> i64 {
    n * (n + 1) * (2 * n + 1) / 6
}

// c + 2c + 3c +...max以下最大の数まで
fn tousa_sum(max: i64, c: i64) -> i64 {
    let n = max / c;
    (c + n * c) * n / 2
}

// 1 + c + c^2 + c^3 + c^4+...(0<c<1)
fn sum_inf(c: f64) -> f64 {
    1.0 / (1.0 - c)
}

// n!, 1/n, 1/n! をかえす
fn fact_inv(n: usize, modulo: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut fact = vec![0; n + 1];
    let mut inv = vec![0; n + 1];
    let mut fact_inv = vec![0; n + 1];
    fact[0] = 1;
    fact[1] = 1;
    inv[1] = 1;
    fact_inv[0] = 1;
    fact_inv[1] = 1;
    for i in 2..(n + 1) {
        fact[i] = fact[i - 1] * i % modulo;
        inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
        fact_inv[i] = fact_inv[i - 1] * inv[i] % modulo;
    }
    (fact, inv, fact_inv)
}

// 引数の約数を全て返却する。
// 計算量はO(√N)
fn divisor(n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut upper = vec![];
    for i in 1..=n.sqrt() {
        if n % i == 0 {
            res.push(i);
            if i != n / i {
                upper.push(n / i);
            }
        }
    }
    upper.reverse();
    res.append(&mut upper);
    res
}

// 最大公約数
// ユークリッドの互除法
// 計算量はO(log min(a,b))
fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

// 最小公倍数
// 計算量はO(log min(a,b))
fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

// 拡張ユークリッドの互除法
// ax + by = gcd(a,b) の整数解を求める
// 参照を以下関数に渡す
fn extend_euclid(a: i64, b: i64, x: &mut i64, y: &mut i64) -> i64 {
    if b == 0 {
        *x = 1;
        *y = 0;
        return a;
    }
    let d = extend_euclid(b, a % b, y, x);
    *y -= a / b * *x;
    d
}

// x^n を求める
// 計算量は O(logn)
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

// 逆元を求める。(moduloが素数でなくてもOK)
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

// いづれかの倍数であるものの数
// https://atcoder.jp/contests/math-and-algorithm/tasks/math_and_algorithm_be
fn num_multiple(n: i64, k: usize, v: Vec<i64>) -> i64 {
    let mut ans = 0_i64;
    for bits in 1..1 << k {
        // 包除原理。偶数個のものはひいて、奇数個のものは足す。
        let mut is_even = true;
        let mut x = 1;
        for (i, v_i) in v.iter().copied().enumerate() {
            if ((bits >> i) & 1) == 1 {
                x = lcm(x, v_i);
                is_even = !is_even;
            }
        }
        ans += if is_even { -1 } else { 1 } * (n / x);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor() {
        assert_eq!(divisor(5), vec![1, 5]);
        assert_eq!(divisor(6), vec![1, 2, 3, 6]);
        assert_eq!(divisor(9), vec![1, 3, 9]);
    }

    #[test]
    fn test_extend_euclid() {
        let mut x = 0;
        let mut y = 0;
        extend_euclid(48, 32, &mut x, &mut y);
        assert_eq!(x, 1);
        assert_eq!(y, -1);
    }

    #[test]
    fn test_pow() {
        assert_eq!(pow(3, 4, 100), 81);
        assert_eq!(pow(3, 4, 80), 1);
    }
}
