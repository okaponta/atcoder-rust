use std::collections::HashSet;

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
    let mut a = HashSet::new();
    let mut b = HashSet::new();
    let mut c = HashSet::new();
    for i in 0..s.len() {
        if s[i] == 'A' {
            a.insert(i);
        } else if s[i] == 'B' {
            b.insert(i);
        } else if s[i] == 'C' {
            c.insert(i);
        }
    }
    let mut ans = 0;
    for ai in 0..s.len() {
        for j in 1..s.len() {
            if a.contains(&ai) && b.contains(&(ai + j)) && c.contains(&(ai + 2 * j)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
