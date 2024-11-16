use std::collections::BTreeSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert((i, 1, i));
    }
    let mut count = vec![1; n];
    for _ in 0..q {
        input! {qi:u8}
        if qi == 1 {
            input! {x:Usize1, c:Usize1}
            let &(begin, len, col) = set.range(..(x, n, n)).last().unwrap();
            let mut leng = len;
            let mut begg = begin;
            count[col] -= len;
            set.remove(&(begin, len, col));
            let mut rem = vec![];
            if let Some(&fuga) = set.range(..(begin, 0, 0)).last() {
                if fuga.2 == c {
                    begg = fuga.0;
                    leng += fuga.1;
                    rem.push(fuga);
                    count[c] -= fuga.1;
                }
            }
            if let Some(&fuga) = set.range((begin + 1, 0, 0)..).next() {
                if fuga.2 == c {
                    leng += fuga.1;
                    rem.push(fuga);
                    count[c] -= fuga.1;
                }
            }
            for fuga in rem {
                set.remove(&fuga);
            }
            set.insert((begg, leng, c));
            count[c] += leng;
        } else {
            input! {c:Usize1}
            println!("{}", count[c]);
        }
    }
}
