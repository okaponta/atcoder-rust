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
    for ai in a {
        factorized.push(factorize(ai));
    }
    let mut map = HashMap::new();
    for e in factorized.clone() {
        for (k, v) in e {
            let val = *map.get(&k).unwrap_or(&0).max(&v);
            *map.entry(k).or_insert(0) = val;
        }
    }
    let mut ans = 0;
    for e in factorized {
        let mut tmp = 1;
        for (k, v) in &map {
            let mut val = *v;
            for fact in &e {
                if fact.0 == *k {
                    val -= fact.1;
                }
            }
            tmp = (tmp * pow(*k, val, MOD)) % MOD;
        }
        ans = (ans + tmp) % MOD;
    }
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
