#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:Chars,
    }
    let mut ans = vec![0; n];
    let mut tmp = 0;
    for i in 0..n {
        tmp += (s[i].to_digit(10).unwrap() as usize) * (i + 1);
        ans[i] = tmp;
    }
    let mut rev = vec![];
    let mut rem = 0;
    ans.reverse();
    for i in 0..n {
        let tmp = ans[i] + rem;
        rev.push(tmp % 10);
        rem = tmp / 10;
    }
    rev.reverse();
    if 0 < rem {
        println!("{}{}", rem, rev.iter().join(""));
    } else {
        println!("{}", rev.iter().join(""));
    }
}
