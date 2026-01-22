#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        s:Chars,
    }
    let mut ans = 0.0;
    for i in 0..s.len() {
        for j in i + 2..s.len() {
            if s[i] != 't' || s[j] != 't' {
                continue;
            }
            let mut count = 0;
            for k in i + 1..j {
                if s[k] == 't' {
                    count += 1;
                }
            }
            if ans < count as f64 / (j - i - 1) as f64 {
                ans = count as f64 / (j - i - 1) as f64;
            }
        }
    }
    println!("{}", ans);
}
