use proconio::{marker::*, *};
use std::collections::HashSet;

fn main() {
    input! {
        _n:usize,
        m:usize,
        uv:[(Usize1,Usize1);m],
    }
    let mut ans = 0;
    let mut set = HashSet::new();
    for (u, v) in uv {
        if u == v || set.contains(&(u, v)) {
            ans += 1;
            continue;
        }
        set.insert((u, v));
        set.insert((v, u));
    }
    println!("{}", ans);
}
