use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: i32,
        a:[i32;n],
        b:[i32;n],
    }
    let mut amax = 0;
    let mut bmin = 1000;
    for e in a {
        amax = cmp::max(amax, e)
    }
    for e in b {
        bmin = cmp::min(bmin, e)
    }
    println!("{}", if bmin < amax { 0 } else { bmin - amax + 1 })
}
