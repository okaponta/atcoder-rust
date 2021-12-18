use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
       n:usize,k:Usize1,
       p:[(i32,i32,i32);n],
    }
    let points = p.iter().map(|p| p.0 + p.1 + p.2).collect::<Vec<_>>();
    let base = points.iter().sorted().rev().nth(k).unwrap() - 300;
    points
        .iter()
        .for_each(|p| println!("{}", if p >= &base { "Yes" } else { "No" }));
}
