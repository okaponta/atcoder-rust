use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        t:usize,
        c:[usize;n],
        r:[usize;n],
    }
    let mut map = HashMap::new();
    for i in 0..n {
        (*map.entry(c[i]).or_insert(vec![])).push((r[i], i));
    }
    let v = map.get(&t).unwrap_or(&map[&c[0]]);
    let mut ans = 0;
    let mut tmp = 0;
    for &(a, b) in v {
        if tmp < a {
            ans = b;
            tmp = a;
        }
    }
    println!("{}", ans + 1);
}
