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
        s:[Chars;h],
    }
    let mut p1x = h;
    let mut p1y = w;
    let mut p2x = 0;
    let mut p2y = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                p1x = p1x.min(i);
                p1y = p1y.min(j);
                p2x = p2x.max(i);
                p2y = p2y.max(j);
            }
        }
    }
    for i in p1x..=p2x {
        for j in p1y..=p2y {
            if s[i][j] == '.' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
