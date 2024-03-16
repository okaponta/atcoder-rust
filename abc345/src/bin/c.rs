use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let n = s.len();
    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry(s[i]).or_insert(0) += 1;
    }
    let mut ans = n * (n - 1) / 2;
    let mut flg = false;
    for (_, v) in map {
        if 1 < v {
            flg = true;
        }
        ans -= v * (v - 1) / 2;
    }
    if flg {
        ans += 1;
    }
    println!("{}", ans);
}
