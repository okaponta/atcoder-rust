use std::collections::HashMap;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let s = a.iter().sum::<usize>();
    let mut div = divisor(s);
    div.sort();
    div.reverse();
    for d in div {
        let mut prev = HashMap::new();
        prev.insert(0usize, 0usize);
        for &ai in &a {
            let a = ai - ai % d;
            let b = ai + d - ai % d;
            let mut new = HashMap::new();
            for (key, v) in prev {
                if 2 * k < v {
                    continue;
                }
                new.insert(
                    key + a,
                    *new.get(&(key + a))
                        .unwrap_or(&(v + ai % d))
                        .min(&(v + ai % d)),
                );
                new.insert(
                    key + b,
                    *new.get(&(key + b))
                        .unwrap_or(&(v + d - ai % d))
                        .min(&(v + d - ai % d)),
                );
            }
            prev = new;
        }
        if *prev.get(&s).unwrap_or(&(2 * k + 1)) <= 2 * k {
            println!("{}", d);
            return;
        }
    }
}

fn divisor(n: usize) -> Vec<usize> {
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
