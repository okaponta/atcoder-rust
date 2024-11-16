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
    let mut ans = vec![];
    let mut tmp = 0;
    for i in 1..s.len() {
        if s[i] == '-' {
            tmp += 1;
        } else {
            ans.push(tmp);
            tmp = 0;
        }
    }
    println!("{}", ans.iter().join(" "));
}
