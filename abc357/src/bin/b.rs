use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let small = s.iter().filter(|c| c.is_ascii_lowercase()).count();
    let big = s.iter().filter(|c| c.is_ascii_uppercase()).count();
    if small < big {
        println!("{}", s.iter().map(|c| c.to_ascii_uppercase()).join(""));
    } else {
        println!("{}", s.iter().map(|c| c.to_ascii_lowercase()).join(""));
    }
}
