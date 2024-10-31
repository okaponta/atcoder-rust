use std::str::FromStr;

#[allow(unused)]
use itertools::*;
use num::BigInt;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:Usize1,
        k:String,
        b:[Usize1;n],
    }
    let bk = BigInt::from_str(&k).unwrap();
    let ans = index_with_loop(a, bk, b);
    println!("{}", ans + 1);
}

fn index_with_loop(init: usize, k: BigInt, b: Vec<usize>) -> usize {
    let mut v = vec![];
    let mut set = std::collections::HashSet::new();
    let mut tmp = init;
    while !set.contains(&tmp) {
        v.push(tmp);
        set.insert(tmp);
        tmp = b[tmp];
    }
    if k < usize_to_bint(v.len()) {
        return v[bint_to_usize(k)];
    }
    let offset = v.iter().position(|&x| x == tmp).unwrap();
    let l = usize_to_bint(v.len() - offset);
    let idx = offset + (k - offset) % l;
    v[bint_to_usize(idx)]
}

fn usize_to_bint(a: usize) -> BigInt {
    BigInt::new(num_bigint::Sign::Plus, vec![a as u32])
}

fn bint_to_usize(a: BigInt) -> usize {
    if a.to_u32_digits().0 == num_bigint::Sign::NoSign {
        0
    } else {
        a.to_u32_digits().1[0] as usize
    }
}
