#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*,num_integer::*};

#[fastout]
fn main() {
    input! {
        q:usize,
        a:[usize;q],
    }
    let primes = prime_lists(1_000_000);
    let mut set = BTreeSet::new();
    for i in 0..primes.len() {
        let mut tmpa = primes[i] * primes[i];
        if 1_000_000 < tmpa {
            break;
        }
        while tmpa < 1_000_000_000_000 {
            for j in i + 1..primes.len() {
                let mut tmpb = primes[j] * primes[j];
                if 1_000_000_000_000 < tmpa * tmpb {
                    break;
                }
                while tmpa * tmpb <= 1_000_000_000_000 {
                    set.insert((tmpa * tmpb) as usize);
                    tmpb *= primes[j] * primes[j];
                }
            }
            tmpa *= primes[i] * primes[i];
        }
    }
    for a in a {
        println!("{}", set.range(..=a).last().unwrap());
    }
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

// Nまでの数字の素数を返却する
fn prime_lists(n: usize) -> Vec<u128> {
    judge_primes(n)
        .iter()
        .enumerate()
        .filter(|(_, &prime)| prime)
        .map(|(i, _)| i as u128)
        .collect()
}
