#[allow(unused)]
use itertools::*;
use num_integer::Roots;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
    }
    let primes = primes(2000000);
    let mut two = vec![];
    for &p in &primes {
        two.push(p * p);
    }
    let mut ans = 0;
    for i in 0..two.len() {
        if n < two[i] {
            break;
        }
        for j in 0..i {
            if n < two[i] * two[j] {
                break;
            }
            ans += 1;
        }
    }
    for &p in &two {
        if n < p * p * p * p {
            break;
        }
        ans += 1;
    }
    println!("{}", ans);
}

fn primes(n: usize) -> Vec<usize> {
    let mut prime = vec![true; n + 1];
    prime[0] = false;
    prime[1] = false;
    for i in 2..=n.sqrt() {
        if !prime[i] {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            prime[j] = false;
        }
    }
    let mut res = vec![];
    for i in 0..=n {
        if prime[i] {
            res.push(i);
        }
    }
    res
}
