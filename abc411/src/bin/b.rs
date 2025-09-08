#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        d:[usize;n-1],
    }
    for i in 0..n - 1 {
        let mut ans = vec![];
        for j in i + 1..n {
            let mut tmp = 0;
            for k in i..j {
                tmp += d[k];
            }
            ans.push(tmp);
        }
        println!("{}", ans.iter().join(" "))
    }
}
