#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        qr:[(usize,usize);n],
        q:usize,
        td:[(Usize1,usize);q],
    }
    for (t, d) in td {
        let (q, r) = qr[t];
        if r == 0 {
            println!("{}", ((q + d - 1) / q) * q);
        } else if r < d {
            println!("{}", ((d - r + q - 1) / q) * q + r);
        } else {
            println!("{}", r);
        }
    }
}
