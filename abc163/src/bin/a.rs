use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        r:f64,
    }
    println!("{}", 2.0 * r * PI);
}
