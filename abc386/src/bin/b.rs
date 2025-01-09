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
    s.reverse();
    let mut ans = 0;
    while 0 < s.len() {
        if 1 < s.len() && s[0] == '0' && s[1] == '0' {
            s.remove(0);
        }
        ans += 1;
        s.remove(0);
    }
    println!("{}", ans);
}
