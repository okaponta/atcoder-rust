#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        mut c:[usize;n],
    }
    if n == 1 {
        println!("{}", c[0] * 2 % MOD);
        return;
    }
    c.sort();
    c.reverse();
    let b = pow(2, n - 1, MOD);
    let mut ans = 0;
    for i in 0..n {
        ans += (((i + 2) * b) % MOD) * c[i];
        ans %= MOD;
    }
    println!("{}", (ans * b) % MOD);
}

fn pow(mut x: usize, mut n: usize, modulo: usize) -> usize {
    x %= modulo;
    let mut ret = if x == 0 { 0 } else { 1 };
    while n > 0 {
        if n & 1 == 1 {
            ret = ret * x % modulo;
        }
        x = x * x % modulo;
        n >>= 1;
    }
    ret
}
