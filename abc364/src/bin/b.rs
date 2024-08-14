#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        h:usize,
        w:usize,
        si:Usize1,
        sj:Usize1,
        c:[Chars;h],
        x:Chars,
    }
    let mut i = si;
    let mut j = sj;
    for x in x {
        let mut ni = i;
        let mut nj = j;
        if x == 'L' {
            nj = nj.wrapping_add(!0);
        } else if x == 'R' {
            nj += 1;
        } else if x == 'U' {
            ni = ni.wrapping_add(!0);
        } else {
            ni += 1;
        }
        if h <= ni || w <= nj || c[ni][nj] == '#' {
            continue;
        }
        i = ni;
        j = nj;
    }
    println!("{} {}", i + 1, j + 1);
}
