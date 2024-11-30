#[allow(unused)]
use itertools::*;
#[allow(unused)]
use proconio::{marker::*, *};
#[allow(unused)]
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    let o = m - (n - 1) * 10 - 1;
    let cnt = binom(o + n, n);
    println!("{}", cnt);
    let mut ans = vec![1; n];
    for i in 1..n {
        ans[i] = ans[i - 1] + 10;
    }
    println!("{}", ans.iter().join(" "));
    while next(&mut ans, m, n) {
        println!("{}", ans.iter().join(" "));
    }
}

fn next(a: &mut Vec<usize>, m: usize, n: usize) -> bool {
    let mut tmp = n - 1;
    loop {
        if tmp == n - 1 && a[tmp] != m {
            a[tmp] += 1;
            return true;
        } else if tmp != n - 1 && a[tmp] + 10 < a[tmp + 1] {
            a[tmp] += 1;
            for i in tmp + 1..n {
                a[i] = a[i - 1] + 10;
            }
            return true;
        }
        if tmp == 0 {
            return false;
        }
        tmp -= 1;
    }
}

fn binom(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    (0..k).fold(1, |s, k| s * (n - k) / (k + 1))
}
