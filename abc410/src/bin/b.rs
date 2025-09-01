#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        q:usize,
        x:[usize;q],
    }
    let mut count = vec![0; n];
    let mut min = 0;
    let mut ans = vec![];
    for x in x {
        if x == 0 {
            for i in 0..n {
                if count[i] == min {
                    ans.push(i + 1);
                    count[i] += 1;
                    break;
                }
            }
        } else {
            count[x - 1] += 1;
            ans.push(x);
        }
        min = *count.iter().min().unwrap();
    }
    println!("{}", ans.iter().join(" "));
}
