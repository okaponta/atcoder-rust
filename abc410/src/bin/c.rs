#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut r = 0;
    let mut a = (1..=n).into_iter().collect_vec();
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {p:Usize1, x:usize}
            a[(n + p + r) % n] = x;
        } else if qi == 2 {
            input! {p:Usize1}
            println!("{}", a[(n + p + r) % n]);
        } else {
            input! {k :usize}
            r = (r + k) % n;
        }
    }
}
