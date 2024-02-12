use itertools::Itertools;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        b: usize,
    }
    let mut set = HashSet::new();
    for digits in (0..=9).combinations_with_replacement(11) {
        set.insert(digits.iter().product());
    }
    let mut ans = 0;
    for x in set {
        let m = x + b;
        if m <= n && product(m) == x {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn product(mut x: usize) -> usize {
    let mut res = 1;
    while x > 0 {
        res *= x % 10;
        x /= 10;
    }
    res
}
