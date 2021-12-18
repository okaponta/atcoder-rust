use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n:usize,mut x:usize,
       mut a:[usize;n]
    }
    a.insert(0, 0);
    let mut ans: HashSet<usize> = HashSet::new();
    while !ans.contains(&x) {
        ans.insert(x);
        x = a[x];
    }
    println!("{}", ans.len());
}
