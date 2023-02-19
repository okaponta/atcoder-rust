use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    let mut ans = 0;
    for i in 0..n {
        (*map.entry(a[i]).or_insert(vec![])).push(i);
        ans += ((i + 1) / 2) * (n - i);
    }
    for (_, v) in map {
        ans -= calc(v, n);
    }
    println!("{}", ans);
}

fn calc(v: Vec<usize>, n: usize) -> usize {
    let mut res = 0;
    let mut l = 0;
    let mut r = v.len() - 1;
    while l != r {
        let posl = v[l] + 1;
        let posr = n - v[r];
        if posl < posr {
            res += posl * (r - l);
            l += 1;
        } else {
            res += posr * (r - l);
            r -= 1;
        }
    }
    res
}
