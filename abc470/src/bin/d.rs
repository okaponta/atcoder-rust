#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*,std::vec};
use std::println;

fn main() {
    input! {
        n:usize,
        q:usize,
        mut p:[Usize1;n],
    }
    let mut pp = vec![0; n];
    for i in 0..n {
        pp[p[i]] = i;
    }
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {x:Usize1, y:Usize1}
            pp.swap(p[x], p[y]);
            p.swap(x, y);
        } else {
            (p, pp) = (pp, p);
        }
    }
    println!("{}", p.into_iter().map(|i| i + 1).join(" "));
}
