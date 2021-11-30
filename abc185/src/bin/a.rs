use std::cmp::min;

use proconio::input;

fn main() {
    input! {
       a:[i32;4],
    }
    let mut m = 101;
    for e in a {
        m = min(m, e);
    }
    println!("{}", m);
}
