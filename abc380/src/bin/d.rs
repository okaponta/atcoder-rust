#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        s:Chars,
        q:usize,
        k:[Usize1;q],
    }
    let m = s.len();
    let mut ans = vec![];
    for k in k {
        let kk = k / m;
        if kk.count_ones() % 2 == 0 {
            ans.push(s[k % m]);
        } else {
            ans.push(rev(s[k % m]));
        }
    }
    println!("{}", ans.iter().join(" "));
}

fn rev(c: char) -> char {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}
