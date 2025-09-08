#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        _n:usize,
        q:usize,
    }
    let mut query = vec![];
    for _ in 0..q {
        input! {qi:u8, p:usize}
        if qi == 2 {
            input! {s: Chars}
            query.push((qi, p, s));
        } else {
            query.push((qi, p, vec![]));
        }
    }
    query.reverse();
    let mut ans = vec![];
    let mut target = 0;
    for (qi, p, s) in query {
        if qi == 1 {
            if target == p {
                target = 0;
            }
        } else if qi == 2 {
            if target == p {
                for c in s.iter().rev() {
                    ans.push(*c);
                }
            }
        } else {
            if target == 0 {
                target = p;
            }
        }
    }
    println!("{}", ans.iter().rev().join(""));
}
