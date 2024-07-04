use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
        w:[usize;n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        map.entry(a[i]).or_insert(vec![]).push(w[i]);
    }
    let mut ans = 0;
    for (_, mut v) in map {
        if 1 < v.len() {
            v.sort();
            ans += (0..v.len() - 1).fold(0, |s, i| s + v[i]);
        }
    }
    println!("{}", ans);
}
