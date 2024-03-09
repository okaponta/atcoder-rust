use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:[usize;n],
        m:usize,
        b:[usize;m],
        l:usize,
        c:[usize;l],
        q:usize,
        x:[usize;q],
    }
    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                set.insert(a[i] + b[j] + c[k]);
            }
        }
    }
    for x in x {
        println!("{}", if set.contains(&x) { "Yes" } else { "No" });
    }
}
