use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        mut n:usize,
        k:usize,
        d:[char;k],
    }
    let mut ngs = HashSet::new();
    for d in d {
        ngs.insert(d);
    }
    while ng(n, &ngs) {
        n += 1;
    }
    println!("{}", n);
}

fn ng(n: usize, ngs: &HashSet<char>) -> bool {
    n.to_string().chars().into_iter().any(|c| ngs.contains(&c))
}
