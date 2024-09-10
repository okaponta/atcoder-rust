#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        mut s:Chars,
        t:Chars,
    }
    let n = s.len();
    let mut ans = vec![];
    while s != t {
        let mut tmp = vec![];
        for i in 0..n {
            if s[i] != t[i] {
                let mut u = s.clone();
                u[i] = t[i];
                tmp.push(u);
            }
        }
        tmp.sort();
        ans.push(tmp[0].iter().collect::<String>());
        s = tmp[0].clone();
    }
    println!("{}", ans.len());
    for ans in ans {
        println!("{}", ans);
    }
}
