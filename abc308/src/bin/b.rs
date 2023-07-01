use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        c:[String;n],
        d:[String;m],
        p:[usize;m+1],
    }
    let mut map = HashMap::new();
    for i in 0..m {
        map.insert(&d[i], p[i + 1]);
    }
    let mut ans = 0;
    for i in 0..n {
        ans += map.get(&c[i]).unwrap_or(&p[0]);
    }
    println!("{}", ans);
}
