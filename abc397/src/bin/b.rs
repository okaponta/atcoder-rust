#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        mut s:Chars,
    }
    let mut ans = 0;
    let mut i = 0;
    while i < s.len() {
        if i % 2 == 0 {
            if s[i] != 'i' {
                s.insert(i, 'i');
                ans += 1;
                continue;
            }
        } else {
            if s[i] != 'o' {
                s.insert(i, 'o');
                ans += 1;
                continue;
            }
        }
        i += 1;
    }
    if s.len() % 2 != 0 {
        ans += 1;
    }
    println!("{}", ans);
}
