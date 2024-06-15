use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        h:usize,
        _w:usize,
        c:[Chars;h],
    }
    for i in 0..2 * h {
        println!("{}", c[i / 2].iter().join(""));
    }
}
