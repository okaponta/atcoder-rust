use itertools::*;
use proconio::{marker::*, *};

fn main() {
    input! {
        n:usize,
        _l:Chars,
        mut l:[String;n],
    }
    l.sort();
    println!("{}", l.iter().join(""));
}
