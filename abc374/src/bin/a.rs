#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut s:Chars,
    }
    s.reverse();
    println!(
        "{}",
        if s[0] == 'n' && s[1] == 'a' && s[2] == 's' {
            "Yes"
        } else {
            "No"
        }
    );
}
