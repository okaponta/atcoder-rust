#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:Chars,
        t:Chars,
    }
    let mut c = vec![];
    for i in 1..s.len() {
        if s[i].is_ascii_uppercase() {
            c.push(s[i - 1]);
        }
    }
    println!(
        "{}",
        if c.iter().all(|c| t.contains(c)) {
            "Yes"
        } else {
            "No"
        }
    );
}
