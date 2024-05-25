use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        b:[usize;m],
    }
    let mut set = HashSet::new();
    let mut c = vec![];
    for i in 0..n {
        c.push(a[i]);
        set.insert(a[i]);
    }
    for i in 0..m {
        c.push(b[i]);
    }
    c.sort();
    for i in 1..c.len() {
        if set.contains(&c[i - 1]) && set.contains(&c[i]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
