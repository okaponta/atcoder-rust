#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        t:Chars,
        u:Chars,
    }
    for i in 0..=t.len() - u.len() {
        let mut ok = true;
        for j in 0..u.len() {
            if t[i + j] != '?' && t[i + j] != u[j] {
                ok = false;
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
