#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut s:Chars,
    }
    s.push('+');
    let mut ans = 0;
    let mut tmp = 1;
    for c in s {
        if c == '+' {
            if tmp != 0 {
                ans += 1;
            }
            tmp = 1;
        } else if c == '0' {
            tmp = 0;
        }
    }
    println!("{}", ans);
}
