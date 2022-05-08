use num_integer::Roots;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
    }
    let primes = judge_primes(1_000_000);
    let mut prime_num = vec![];
    for i in 0..1_000_000 {
        if primes[i] {
            prime_num.push(i);
        }
    }
    let mut ans = 0;
    for q in prime_num.clone() {
        if q * q * q > n {
            break;
        }
        let max = (n / q / q / q).min(q - 1);
        ans += prime_num.upper_bound(&max);
    }
    println!("{}", ans);
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
