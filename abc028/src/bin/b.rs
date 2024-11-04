#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = vec![0; 6];
    for c in s {
        ans[(c as u8 - b'A') as usize] += 1;
    }
    println!("{}", ans.iter().join(" "));
}
