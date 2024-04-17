use std::collections::HashMap;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let mut map = HashMap::new();
    for i in 1..=n {
        for (j, k) in factorize(i) {
            *map.entry(j).or_insert(0) += k;
        }
    }
    let ans = map
        .into_iter()
        .fold(1, |s, (_, v)| (s * (v + 1)) % 1_000_000_007);
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
