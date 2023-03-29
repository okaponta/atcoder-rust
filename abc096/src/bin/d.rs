use num_integer::Roots;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
    }
    let primes = prime_lists_one(3100000);
    for i in 0..n {
        print!("{} ", primes[i]);
    }
    println!();
}

fn prime_lists_one(n: usize) -> Vec<usize> {
    judge_primes(n)
        .iter()
        .enumerate()
        .filter(|(i, &prime)| prime && i % 5 == 1)
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
