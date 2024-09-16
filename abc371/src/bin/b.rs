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
        m:usize,
        ab:[(Usize1,char);m],
    }
    let mut flg = vec![false; n];
    for (a, b) in ab {
        if b == 'F' {
            println!("No");
        } else {
            if flg[a] {
                println!("No");
            } else {
                println!("Yes");
                flg[a] = true;
            }
        }
    }
}
