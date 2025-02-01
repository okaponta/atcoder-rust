#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        s:[Chars;n],
        t:[Chars;m],
    }
    for i in 0..=n - m {
        for j in 0..=n - m {
            let mut ok = true;
            for k in 0..m {
                for l in 0..m {
                    if s[i + k][j + l] != t[k][l] {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}
