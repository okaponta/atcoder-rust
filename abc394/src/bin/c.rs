use itertools::*;
use proconio::{marker::*, *};

fn main() {
    input! {
        mut s:Chars,
    }
    let mut i = 0;
    while i + 1 < s.len() {
        if s[i] == 'W' && s[i + 1] == 'A' {
            s[i] = 'A';
            s[i + 1] = 'C';
            i = i.saturating_sub(1);
        } else {
            i += 1;
        }
    }
    println!("{}", s.iter().join(""));
}
