use num_integer::Roots;

// 引数の約数を全て返却する。
// 計算量はO(√N)
fn divisor(n: i32) -> Vec<i32> {
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
fn factorize(mut n: i64) -> Vec<(i64, i32)> {
    let mut res = vec![];
    for a in 2..=n.sqrt() {
        if n % a != 0 {
            continue;
        }
        let mut ex = 0;
        while n % a == 0 {
            ex += 1;
            n /= a;
        }
        res.push((a, ex));
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(divisor(5), vec![1, 5]);
        assert_eq!(divisor(6), vec![1, 2, 3, 6]);
        assert_eq!(divisor(9), vec![1, 3, 9]);
    }
}
