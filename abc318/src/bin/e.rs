use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        (*map.entry(a[i]).or_insert(vec![])).push(i);
    }
    let mut ans = 0;
    for (_, v) in map {
        let mut minus = v[0] + 1;
        for i in 1..v.len() {
            ans += i * v[i] - minus;
            minus += v[i] + 1 + i;
        }
    }
    println!("{}", ans);
}
