use num_integer::Roots;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t:usize,
        n:[usize;t],
    }
    let primes = prime_lists(3_000_000);
    for i in 0..t {
        let mut n = n[i];
        for p in &primes {
            if n % p == 0 {
                n = n / p;
                if n % p == 0 {
                    println!("{} {}", p, n / p);
                } else {
                    println!("{} {}", n.sqrt(), p);
                }
                break;
            }
        }
    }
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
