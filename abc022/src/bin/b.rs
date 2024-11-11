use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut ans = 0;
    let mut set = HashSet::new();
    for a in a {
        if set.contains(&a) {
            ans += 1;
        } else {
            set.insert(a);
        }
    }
    println!("{}", ans);
}
