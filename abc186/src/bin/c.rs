use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n:usize,
    }
    let mut set = HashSet::new();
    for i in 1..=n {
        if i.to_string().contains('7') {
            set.insert(i);
            continue;
        }
        let a = format!("{:o}", i);
        if a.contains('7') {
            set.insert(i);
        }
    }
    println!("{}", n - set.len());
}
