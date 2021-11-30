use std::cmp::min;

use proconio::input;

fn main() {
    input! {
        h:usize,w:usize,
        a:[i32;h*w]
    }
    let mut m = 101;
    let mut sum = 0;
    for i in 0..h * w {
        m = min(m, a[i]);
        sum += a[i];
    }
    println!("{}", sum - m * h as i32 * w as i32);
}
