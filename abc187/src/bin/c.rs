use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
       n:usize,
       s:[String;n],
    }
    let mut set = HashSet::new();
    let mut ex_set = HashSet::new();
    for si in s {
        if si.starts_with("!") {
            ex_set.insert(si[1..].to_string());
        } else {
            set.insert(si);
        }
    }
    for str in set {
        if ex_set.contains(&str) {
            println!("{}", str);
            return;
        }
    }
    println!("satisfiable");
}
