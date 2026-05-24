#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        q:usize,
        qq:[(u8, Usize1);q],
    }
    let mut count = vec![0; n];
    let mut s = vec![0; 1_000_000];
    let mut offset = 0;
    s[0] = n;
    for (qi, i) in qq {
        if qi == 1 {
            let bef = count[i];
            s[bef + 1] += 1;
            count[i] += 1;
            if s[bef + 1] == n {
                offset += 1;
            }
        } else {
            println!("{}", s[i + 1 + offset]);
        }
    }
}
