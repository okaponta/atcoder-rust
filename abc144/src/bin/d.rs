use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        a:f64,
        b:f64,
        x:f64
    }
    let c = b - x / a / a;
    let tan = if 2.0 * c < b {
        2.0 * c / a
    } else {
        a * b * b / (2.0 * x)
    };
    println!("{}", tan.atan() / PI * 180.0);
}
