#[rustfmt::skip]#[allow(unused)]
use {itertools::*,proconio::{marker::*, *},superslice::*,std::collections::*};

fn main() {
    input! {
        a:usize,
        n:usize,
    }
    let mut ans = 0;
    for i in 1..13 {
        for j in 10usize.pow((i - 1) / 2)..10usize.pow((i + 1) / 2) {
            let mut k = j.to_string().chars().collect_vec();
            for l in (0..i / 2).rev() {
                let c = k[l as usize];
                k.push(c);
            }
            let k = k.into_iter().collect::<String>().parse::<usize>().unwrap();
            if n < k {
                break;
            }
            if is_kaibun(base_n(k, a)) {
                ans += k;
            }
        }
    }
    println!("{}", ans);
}

fn base_n(mut n: usize, k: usize) -> Vec<usize> {
    let mut res = vec![];
    while 0 < n {
        res.push(n % k);
        n /= k;
    }
    res
}

fn is_kaibun(s: Vec<usize>) -> bool {
    let l = s.len();
    (0..l).all(|i| s[i] == s[l - 1 - i])
}
