use nalgebra::{Rotation2, Vector2};
use proconio::input;

fn main() {
    input! {
        (a,b,d):(f64,f64,f64)
    }
    let p = Vector2::new(a, b);
    let rot = Rotation2::new(d * 2.0 * std::f64::consts::PI / 360.0);
    let res = rot * p;
    println!("{} {}", res[0], res[1]);
}
