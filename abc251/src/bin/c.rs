use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        st:[(String,i64);n],
    }
    let mut set = HashSet::new();
    let mut point = -1;
    let mut ans = 0;
    for i in 0..n {
        let s = &st[i].0;
        let t = st[i].1;
        if set.contains(&s) {
            continue;
        }
        set.insert(s);
        if point < t {
            point = t;
            ans = i + 1;
        }
    }
    println!("{}", ans);
}
