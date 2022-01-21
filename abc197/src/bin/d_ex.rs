use nalgebra::*;
use proconio::input;

fn main() {
    input! {n: usize, (x0, y0, xi, yi): (f64, f64, f64, f64)}
    let center = Vector2::new((x0 + xi) / 2.0, (y0 + yi) / 2.0);
    let a0 = Vector2::new(x0, y0);
    let rotation = Rotation2::new(2.0 * std::f64::consts::PI / n as f64);
    let result = rotation * (a0 - center) + center;
    println!("{} {}", result[0], result[1]);
}
