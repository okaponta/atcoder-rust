#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    for i in 1..n - 1 {
        if s[i - 1][1] == 'w' && s[i][1] == 'w' {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
