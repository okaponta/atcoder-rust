use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        ma:i64,
        mb:i64,
        abc:[(i64,i64,i64);n],
    }
    let abc = abc
        .into_iter()
        .map(|(a, b, c)| (a * mb, b * ma, c))
        .collect::<Vec<_>>();
    let m = n / 2;
    let mut first = HashMap::new();

    let mut ans = 1 << 60;
    for i in 1..1 << m {
        let mut k = 0;
        let mut v = 0;
        for j in 0..m {
            if i >> j & 1 == 1 {
                k += abc[j].0 - abc[j].1;
                v += abc[j].2;
            }
        }
        if k == 0 {
            ans = ans.min(v);
        } else {
            first.insert(k, *first.get(&k).unwrap_or(&(1 << 60)).min(&v));
        }
    }
    for i in 1..1 << (n - m) {
        let mut k = 0;
        let mut v = 0;
        for j in 0..(n - m) {
            if i >> j & 1 == 1 {
                k += abc[m + j].1 - abc[m + j].0;
                v += abc[m + j].2;
            }
        }
        if k == 0 {
            ans = ans.min(v);
        } else {
            ans = ans.min(v + *first.get(&k).unwrap_or(&(1 << 60)));
        }
    }
    if ans == 1 << 60 {
        println!("-1");
        return;
    }
    println!("{}", ans);
}
