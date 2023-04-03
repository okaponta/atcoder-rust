use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        x:i64,
        a:[i64;n],
    }
    let mut set = HashSet::new();
    for a in a {
        set.insert(a);
        if set.contains(&(a + x)) || set.contains(&(a - x)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
