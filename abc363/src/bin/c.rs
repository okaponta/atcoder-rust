use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        k:usize,
        mut s:Chars,
    }
    s.sort();
    let mut ans = if f(n, k, &s) { 1 } else { 0 };
    while s.next_permutation() {
        if f(n, k, &s) {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn f(n: usize, k: usize, s: &Vec<char>) -> bool {
    (0..=n - k).all(|i| !(0..(k + 1) / 2).all(|j| s[i + j] == s[i + k - 1 - j]))
}
