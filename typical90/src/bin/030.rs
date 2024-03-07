use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    let p = prime_lists(n);
    let mut c = vec![0; n + 1];
    for &i in &p {
        for j in (i..=n).step_by(i) {
            c[j] += 1;
        }
    }
    let mut ans = 0;
    for i in 2..=n {
        if k <= c[i] {
            ans += 1;
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
