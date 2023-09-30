use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;m],
    }
    let mut set = HashSet::new();
    for i in 0..m {
        set.insert(a[i]);
    }
    let mut ans = vec![0; n];
    let mut tmp = 0;
    for i in (1..=n).rev() {
        if set.contains(&i) {
            tmp = 0;
        } else {
            tmp += 1;
            ans[i - 1] = tmp;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
