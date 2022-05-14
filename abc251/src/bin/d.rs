use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _w:usize,
    }
    let mut ans = vec![];
    for i in 1..100 {
        ans.push(i);
    }
    for i in 1..100 {
        ans.push(i * 100);
    }
    for i in 1..=100 {
        ans.push(i * 10000);
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}
