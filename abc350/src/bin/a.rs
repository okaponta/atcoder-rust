use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let mut set = HashSet::new();
    for i in 1..350 {
        if i == 316 {
            continue;
        }
        let mut tmp = vec!['A', 'B', 'C'];
        tmp.push(f(i / 100));
        tmp.push(f((i % 100) / 10));
        tmp.push(f(i % 10));
        set.insert(tmp);
    }
    println!("{}", if set.contains(&s) { "Yes" } else { "No" });
}

fn f(i: usize) -> char {
    std::char::from_digit(i as u32, 10).unwrap()
}
