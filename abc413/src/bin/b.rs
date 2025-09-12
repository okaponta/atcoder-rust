#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut set = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let mut tmp = s[i].clone();
            for &c in &s[j] {
                tmp.push(c);
            }
            set.insert(tmp);
        }
    }
    println!("{}", set.len());
}
