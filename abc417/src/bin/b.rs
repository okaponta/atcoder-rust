#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        m:usize,
        a:[usize;n],
        mut b:[usize;m],
    }
    let mut ans = vec![];
    for a in &a {
        if let Some(i) = b.iter().position(|j| j == a) {
            b.remove(i);
        } else {
            ans.push(a);
        }
    }
    println!("{}", ans.iter().join(" "));
}
