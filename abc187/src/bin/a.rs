use std::cmp::max;

use proconio::input;

fn s(i: i32) -> i32 {
    let ten = (i / 10) % 10;
    return i / 100 + ten + i % 10;
}

fn main() {
    input! {
       a:i32,b:i32,
    }
    println!("{}", max(s(a), s(b)));
}
