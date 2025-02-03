use std::collections::HashSet;

#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        a:[usize;n],
    }
    let mut xor = vec![0; n];
    let mut set = HashSet::new();
    dfs(0, 0, n, &a, &mut xor, &mut set);
    println!("{}", set.len());
}

fn dfs(
    mut s: usize,
    i: usize,
    n: usize,
    a: &Vec<usize>,
    xor: &mut Vec<usize>,
    set: &mut HashSet<usize>,
) {
    if s.count_ones() == n as u32 {
        let mut aa = 0;
        let mut bb = 0;
        for i in 0..n {
            aa += xor[i];
            bb += a[i];
        }
        if aa != bb {
            println!("{:?}", xor);
        }
        let tmp = xor.iter().fold(0, |acc, x| acc ^ x);
        set.insert(tmp);
        return;
    }
    let l = (0..n).into_iter().find(|i| s >> i & 1 == 0).unwrap();
    xor[i] = a[l];
    s |= 1 << l;
    let rem = s ^ ((1 << n) - 1);
    let mut sub = rem;
    while sub > 0 {
        for j in 0..n {
            if sub >> j & 1 == 1 {
                xor[i] += a[j];
            }
        }
        dfs(s | sub, i + 1, n, a, xor, set);
        xor[i] = a[l];
        sub = sub - 1 & rem;
        if sub == rem {
            break;
        }
    }
    dfs(s, i + 1, n, a, xor, set);
    xor[i] = 0;
}
