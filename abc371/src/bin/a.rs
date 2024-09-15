#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        sab:char,
        sac:char,
        sbc:char,
    }
    for v in (0..3).into_iter().permutations(3) {
        if f(v[0], v[1], sab) && f(v[0], v[2], sac) && f(v[1], v[2], sbc) {
            println!(
                "{}",
                (b'A' + v.iter().position(|&i| i == 1).unwrap() as u8) as char
            );
            return;
        }
    }
}

fn f(a: u8, b: u8, c: char) -> bool {
    (a < b && c == '<') || (a > b && c == '>')
}
