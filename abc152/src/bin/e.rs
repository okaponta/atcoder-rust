use std::collections::HashMap;

use num_integer::Roots;
use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        a:[i64;n],
    }
    let mut factorized = vec![];
    for ai in &a {
        factorized.push(factorize(*ai));
    }
    let mut map = HashMap::new();
    for e in factorized {
        for (k, v) in e {
            let val = *map.get(&k).unwrap_or(&0).max(&v);
            *map.entry(k).or_insert(0) = val;
        }
    }
    let lcm = map
        .iter()
        .map(|(k, v)| pow(*k, *v, MOD))
        .fold(1, |acc, x| (acc * x) % MOD);
    let ans = a
        .iter()
        .map(|e| (lcm * modinv(*e, MOD)) % MOD)
        .fold(0, |acc, x| (acc + x) % MOD);
    println!("{}", ans);
}

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
