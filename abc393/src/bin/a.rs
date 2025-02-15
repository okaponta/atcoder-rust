#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s1:Chars,
        s2:Chars,
    }
    let mut ans = 1;
    if s1[0] == 'f' {
        ans += 2;
    }
    if s2[0] == 'f' {
        ans += 1;
    }
    println!("{}", ans);
}
