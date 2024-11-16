#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        _n:usize,
        k:usize,
        s:Chars,
    }
    let is_ev = s[0] == '0';
    let mut rl = run_length_encode(s);
    if is_ev {
        rl.swap(k * 2 - 1, k * 2 - 2);
    } else {
        rl.swap(k * 2 - 2, k * 2 - 3);
    }
    let mut ans = vec![];
    for (c, cnt) in rl {
        for _ in 0..cnt {
            ans.push(c);
        }
    }
    println!("{}", ans.iter().join(""));
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
