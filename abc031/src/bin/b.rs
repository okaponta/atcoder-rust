#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        l:usize,
        h:usize,
        n:usize,
        a:[usize;n],
    }
    for a in a {
        if a < l {
            println!("{}", l - a);
        } else if h < a {
            println!("-1");
        } else {
            println!("0");
        }
    }
}
