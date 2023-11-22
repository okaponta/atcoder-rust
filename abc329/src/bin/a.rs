use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!("{}", s.into_iter().join(" "));
}
