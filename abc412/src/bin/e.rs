#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};
use num_integer::Roots;

fn main() {
    input! {
        l:usize,
        r:usize,
    }
    let primes = prime_lists(r.sqrt());
    let mut used = vec![false; r - l];
    let mut ans = 1;
    for p in primes {
        for i in (((l / p) + 1) * p..=r).step_by(p) {
            if !used[i - l - 1] {
                used[i - l - 1] = true;
                let mut x = i;
                while x % p == 0 {
                    x /= p;
                }
                if x == 1 {
                    ans += 1;
                }
            }
        }
    }
    ans += used.iter().filter(|&&b| !b).count();
    println!("{}", ans);
}

fn prime_lists(n: usize) -> Vec<usize> {
    judge_primes(n)
        .iter()
        .enumerate()
        .filter(|(_, &prime)| prime)
        .map(|(i, _)| i)
        .collect()
}

fn judge_primes(n: usize) -> Vec<bool> {
    let mut res = vec![true; n + 1];
    res[0] = false;
    res[1] = false;
    for i in 2..=n.sqrt() {
        if !res[i] {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            res[j] = false;
        }
    }
    res
}
