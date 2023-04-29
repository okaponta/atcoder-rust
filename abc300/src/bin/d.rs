use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let primes = prime_lists(1000000);
    let mut ans = 0;
    for k in (0..primes.len()).rev() {
        let cc = primes[k] * primes[k];
        if n < cc {
            continue;
        }
        for i in 0..k {
            let aa = primes[i] * primes[i];
            if n < aa * cc {
                break;
            }
            for j in i + 1..k {
                let b = primes[j];
                if aa * b * cc <= n {
                    ans += 1;
                } else {
                    break;
                }
            }
        }
    }
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
