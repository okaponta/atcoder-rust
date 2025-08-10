#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        mut s:Chars,
    }
    s.reverse();
    let mut ans = 0;
    let mut count = 0;
    for c in s {
        let ci = c.to_digit(10).unwrap() as usize;
        while count != ci {
            count += 1;
            count %= 10;
            ans += 1;
        }
        ans += 1;
    }
    println!("{}", ans);
}
