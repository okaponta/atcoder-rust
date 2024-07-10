use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a:usize,
        b:usize,
        c:usize,
    }
    let mut set = HashSet::new();
    set.insert(a);
    set.insert(b);
    set.insert(c);
    println!("{}", set.len());
}
