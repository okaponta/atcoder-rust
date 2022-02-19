use std::collections::HashSet;

use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        a:usize,b:usize,c:usize,d:usize,
    }
    let primes = prime_lists(200);
    let mut set = HashSet::new();
    for prime in primes {
        set.insert(prime);
    }
    for i in a..=b {
        let mut aoki_can_win = false;
        for j in c..=d {
            if set.contains(&(i + j)) {
                aoki_can_win = true;
            }
        }
        if !aoki_can_win {
            println!("Takahashi");
            return;
        }
    }
    println!("Aoki");
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
