use num_integer::Roots;
use proconio::input;

fn main() {
    input! {
       n:usize,
       a:[usize;n]
    }
    let mut gcd = a[0];
    let mut is_pairwise = true;
    let mut primes = vec![false; 1_000_001];
    let p = eratosthenes(1_000_001);
    for mut ai in a {
        gcd = calc_gcd(gcd, ai);
        let mut factors = vec![];
        while ai != 1 {
            let factor = if p[ai] == 0 { ai } else { p[ai] };
            factors.push(factor);
            ai /= factor;
        }
        factors.sort();
        factors.dedup();
        for factor in factors {
            if primes[factor] {
                is_pairwise = false;
            }
            primes[factor] = true;
        }
    }
    if gcd != 1 {
        println!("not coprime");
        return;
    }
    if is_pairwise {
        println!("pairwise coprime");
        return;
    }
    println!("setwise coprime");
}

fn eratosthenes(n: usize) -> Vec<usize> {
    let mut res = vec![0; n + 1];
    res[0] = 0;
    res[1] = 1;
    for i in 2..=n.sqrt() {
        if res[i] != 0 {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            res[j] = i;
        }
    }
    res
}

fn calc_gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => calc_gcd(b, a % b),
    }
}
