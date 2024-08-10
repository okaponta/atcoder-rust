#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        s:[Chars;n],
    }
    let mut m = 0;
    for i in 0..n {
        m = m.max(s[i].len());
    }
    for i in 0..m {
        let mut tmp = vec![];
        for j in (0..n).rev() {
            if i < s[j].len() {
                tmp.push(s[j][i]);
            } else {
                tmp.push('*');
            }
        }
        while tmp[tmp.len() - 1] == '*' {
            tmp.pop();
        }
        println!("{}", tmp.iter().join(""));
    }
}
