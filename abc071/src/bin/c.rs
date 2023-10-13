use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut set = HashSet::new();
    let mut pair = vec![];
    for a in a {
        if set.contains(&a) {
            set.remove(&a);
            pair.push(a);
        } else {
            set.insert(a);
        }
    }
    if pair.len() < 2 {
        println!("0");
        return;
    }
    pair.sort();
    pair.reverse();
    println!("{}", pair[0] * pair[1]);
}
