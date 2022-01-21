use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    print!("{}", calc(&a, 0, 0, 0));
}

fn calc(a: &Vec<u64>, ans: u64, s: u64, index: usize) -> u64 {
    if index == a.len() {
        return ans ^ s;
    }
    let ret = calc(a, ans, s | a[index], index + 1);
    let b = calc(a, ans ^ s, a[index], index + 1);
    min(b, ret)
}
