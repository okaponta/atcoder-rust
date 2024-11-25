#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        _n:usize,
        s:Chars,
    }
    let rl = run_length_encode(s);
    if rl.len() < 2 {
        println!("1");
        return;
    }
    let mut ans = 1;
    for i in 0..rl.len() - 2 {
        if rl[i].0 == '1' && rl[i + 1].0 == '/' && rl[i + 1].1 == 1 && rl[i + 2].0 == '2' {
            ans = ans.max(rl[i].1.min(rl[i + 2].1) * 2 + 1);
        }
    }
    println!("{}", ans);
}

fn run_length_encode<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });
    a
}
