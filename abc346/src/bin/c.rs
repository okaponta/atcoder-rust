use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
    }
    let mut ans = k * (k + 1) / 2;
    let mut set = HashSet::new();
    for a in a {
        if a <= k {
            set.insert(a);
        }
    }
    for i in set {
        ans -= i;
    }
    println!("{}", ans);
}
