#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut x:Chars,
    }
    let mut tmp = false;
    while let Some(c) = x.pop() {
        if c == 'h' && !tmp {
            tmp = true;
        } else if c == 'c' && tmp {
            tmp = false;
        } else if c == 'o' || c == 'k' || c == 'u' {
        } else {
            println!("NO");
            return;
        }
    }
    if tmp {
        println!("NO");
    } else {
        println!("YES");
    }
}
