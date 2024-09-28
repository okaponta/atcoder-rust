use std::collections::HashMap;

use itertools::*;
use proconio::*;
use superslice::*;

fn main() {
    input! {
        n:usize,
        m:usize,
        k:i64,
        a:[i64;n],
    }
    if n == m {
        println!("{}", (0..n).into_iter().map(|_i| 0).join(" "));
        return;
    }
    let mut b = a.clone();
    b.sort();

    let cand = (0..=m).into_iter().map(|i| b[n - m - 1 + i]).collect_vec();
    let s = ruiseki(&cand);
    let rem = k - (0..n).into_iter().map(|i| b[i]).sum::<i64>();

    let mut map = HashMap::new();
    for i in 0..n {
        if b[i] + rem < cand[1] {
            map.insert(b[i], -1);
            continue;
        }
        map.insert(b[i], f(b[i], rem, &cand, &s));
    }
    println!("{}", (0..n).into_iter().map(|i| map[&a[i]]).join(" "));
}

fn f(now: i64, rem: i64, cand: &Vec<i64>, s: &Vec<i64>) -> i64 {
    let mut lower = now - 1;
    let mut upper = now + rem + 1;
    while upper - lower > 1 {
        let med = (lower + upper) / 2;
        if is_ok(now, med, rem, cand, s) {
            upper = med;
        } else {
            lower = med;
        }
    }
    upper - now
}

fn is_ok(now: i64, med: i64, rem: i64, cand: &Vec<i64>, s: &Vec<i64>) -> bool {
    let pos = cand.lower_bound(&(med + 1));
    let need = (med + 1) * (pos as i64 - 1) - s[pos] + cand[0].max(now);
    rem - (med - now) < need
}

fn ruiseki(a: &Vec<i64>) -> Vec<i64> {
    let mut res = vec![0];
    for i in 0..a.len() {
        res.push(res[i] + a[i]);
    }
    res
}
