use proconio::input;
use std::cmp;

fn main() {
    input! {
            a: i32,
            b: i32,
            c: i32
    }
    let mut min = cmp::min(a, b);
    min = cmp::min(min, c);
    let ans = a + b + c - min;
    println!("{}", ans);
}
