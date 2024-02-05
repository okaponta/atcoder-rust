use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        o:Chars,
        e:Chars,
    }
    let mut p = vec![];
    for i in 0..o.len() {
        p.push(o[i]);
        if i < e.len() {
            p.push(e[i]);
        }
    }
    println!("{}", p.iter().join(""));
}
