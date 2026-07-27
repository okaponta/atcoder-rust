#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use std::{println, vec};

fn main() {
    input! {
        m:usize,
        d:usize,
        s:Chars,
    }
    let mut ans = vec![true; m];
    for i in 0..m {
        if s[i] == 'G' {
            for j in i.saturating_sub(d)..(i + d + 1).min(m) {
                ans[j] = false;
            }
        }
    }
    println!("{}", ans.into_iter().filter(|i| *i).count());
}
