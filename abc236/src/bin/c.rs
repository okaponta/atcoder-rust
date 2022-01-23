use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n:usize,m:usize,
       s:[String;n],
       t:[String;m],
    }
    let mut set = HashSet::new();
    for ti in t {
        set.insert(ti);
    }
    for si in s {
        if set.contains(&si) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
