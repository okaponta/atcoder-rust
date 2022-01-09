use num_integer::Roots;

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

// 素因数分解する。(素因数,累乗)の形式で返却する
// 計算量はO(√N)
fn factorize(mut n: i64) -> Vec<(i64, i64)> {
    let mut res = vec![];
    for i in 2..=n.sqrt() {
        if n % i != 0 {
            continue;
        }
        let mut ex = 0;
        while n % i == 0 {
            ex += 1;
            n /= i;
        }
        res.push((i, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

// 素数かどうかを判定する
// 計算量はO(√N)
fn is_prime(n: i64) -> bool {
    for i in 2..=n.sqrt() {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Nまでの数字の素数判定を行う
// res[i]==trueならiは素数
// 計算量はO(NloglogN)
fn judge_primes(n: usize) -> Vec<bool> {
    let mut res = vec![true; n + 1];
    res[0] = false;
    res[1] = false;
    for i in 2..=n.sqrt() {
        if !res[i] {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            res[j] = false;
        }
    }
    res
}

// Nまでの数字の素数を返却する
fn prime_lists(n: usize) -> Vec<usize> {
    judge_primes(n)
        .iter()
        .enumerate()
        .filter(|(_, &prime)| prime)
        .map(|(i, _)| i)
        .collect()
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
    let mut ret = 1;
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
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
    fn test_factorize() {
        assert_eq!(factorize(5), vec![(5, 1)]);
        assert_eq!(factorize(60), vec![(2, 2), (3, 1), (5, 1)]);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(53), true);
        assert_eq!(is_prime(55), false);
    }

    #[test]
    fn test_judge_primes() {
        assert_eq!(
            judge_primes(9),
            vec![false, false, true, true, false, true, false, true, false, false]
        );
    }

    #[test]
    fn test_prime_lists() {
        assert_eq!(prime_lists(28), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
        assert_eq!(prime_lists(29), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
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
