use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,
    }
    let mut a: Vec<_> = s.into_iter().permutations(3).collect();
    a.sort();
    a.dedup();
    println!("{}", a.len());
}
