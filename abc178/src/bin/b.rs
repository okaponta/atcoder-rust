use std::cmp::max;

use proconio::input;

fn main() {
    input! {
        a:i64,b:i64,c:i64,d:i64,
    }
    println!("{}", max(max(a * c, a * d), max(b * c, b * d)));
}
