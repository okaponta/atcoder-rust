use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        mut b:[usize;m],
    }
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((a[i], i));
    }
    b.sort();
    let mut ans = 0;
    for b in b {
        if let Some(&(a, i)) = set.range((b, 0)..).next() {
            ans += a;
            set.remove(&(a, i));
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}
