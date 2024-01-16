use std::f64::consts::PI;

use proconio::input;

fn main() {
    input! {
        t:f64,
        l:f64,
        x:f64,
        y:f64,
        q:usize,
        e:[f64;q],
    }
    for e in e {
        let ang = e / t * 2.0 * PI;
        let z = l / 2.0 - ang.cos() * l / 2.0;
        let w = ((y + ang.sin() * l / 2.0).powi(2) + x.powi(2)).sqrt();
        println!("{}", (z / w).atan() * 180.0 / PI)
    }
}
