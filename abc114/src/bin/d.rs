use std::collections::HashMap;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut map = HashMap::new();
    for i in 1..=n {
        for (f, c) in factorize(i) {
            *map.entry(f).or_insert(1) += c;
        }
    }
    let v = map.into_iter().map(|(_, v)| v).collect::<Vec<_>>();
    let n = v.len();
    let mut ans = 0;
    for i in 0..n {
        if 75 <= v[i] {
            ans += 1;
        }
        for j in 0..n {
            if i == j {
                continue;
            }
            if 3 <= v[i] && 25 <= v[j] {
                ans += 1;
            }
            if 5 <= v[i] && 15 <= v[j] {
                ans += 1;
            }
            for k in j + 1..n {
                if i == k {
                    continue;
                }
                if 3 <= v[i] && 5 <= v[j] && 5 <= v[k] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

fn factorize(mut n: usize) -> Vec<(usize, usize)> {
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
