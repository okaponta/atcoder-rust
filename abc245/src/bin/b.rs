use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
    }
    let mut set = HashSet::new();
    for ai in a {
        set.insert(ai);
    }
    for i in 0..2005 {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
