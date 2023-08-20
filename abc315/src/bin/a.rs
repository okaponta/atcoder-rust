use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut t = vec![];
    let ng = vec!['a', 'i', 'u', 'e', 'o'];
    for c in s {
        if !ng.contains(&c) {
            t.push(c);
        }
    }
    println!("{}", t.iter().join(""));
}
