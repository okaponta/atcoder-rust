use num_integer::Roots;

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

// 区間の素数判定
// bが大きい場合に、√bの素数判定を行い
// 区間内の倍数について判定する
fn judge_seg_primes(a: usize, b: usize) -> Vec<bool> {
    let sq = b.sqrt();
    let mut res = vec![true; b - a];
    let mut small = vec![true; sq + 1];
    small[0] = false;
    small[1] = false;
    for i in 2..=sq {
        if !small[i] {
            continue;
        }
        for j in (i * i..=sq).step_by(i) {
            small[j] = false;
        }
        // aをこえるiの最小の倍数
        let a_min = ((a + i - 1) / i) * i;
        for j in (a_min..b).step_by(i) {
            res[j - a] = false;
        }
    }
    res
}

// Nまでの数字の素数判定を行う
// res[i]=0ならiは素数、そうでなければ最小の素因子
// 計算量はO(NloglogN)
fn eratosthenes(n: usize) -> Vec<usize> {
    let mut res = vec![0; n + 1];
    res[0] = 0;
    res[1] = 1;
    for i in 2..=n.sqrt() {
        if res[i] != 0 {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            res[j] = i;
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

// a^(n-1)≡1 (mod n)を満たす素数以外の数
fn judge_carmichael(n: i64) -> bool {
    let factors = factorize(n);
    if factors.len() < 3 {
        // 少なくとも3個以上の異なる素数の積
        return false;
    }
    for (p, _) in factors {
        if (n - 1) % (p - 1) != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
