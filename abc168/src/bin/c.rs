use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
       a:f64,b:f64,h:f64,m:f64,
    }
    let degree = (((h * 60.0 + m) / 720.0) - (m / 60.0)) * 2.0 * PI;
    let x = a * degree.cos();
    let y = a * degree.sin();
    let ans = ((b - x) * (b - x) + y * y).sqrt();
    println!("{}", ans);
}
