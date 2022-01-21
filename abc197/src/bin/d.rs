use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
       n:f64,
       p:(f64,f64),
       pp:(f64,f64),
    }
    let mid = ((p.0 + pp.0) / 2.0, (p.1 + pp.1) / 2.0);
    let v = (p.0 - mid.0, p.1 - mid.1);
    let degree = 2.0 * PI / n;
    let rot = (
        degree.cos() * v.0 - degree.sin() * v.1,
        degree.sin() * v.0 + degree.cos() * v.1,
    );
    println!("{} {}", mid.0 + rot.0, mid.1 + rot.1);
}
