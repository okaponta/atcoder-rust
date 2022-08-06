use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
        m:usize,
    }
    for v in (1..=m).combinations(n) {
        println!("{}", v.iter().join(" "));
    }
}
