use std::mem::swap;

use proconio::input;

fn main() {
    input! {
        mut a:usize,
        mut b:usize,
    }
    if b < a {
        swap(&mut a, &mut b);
    }
    let a = a as f64;
    let b = b as f64;
    if a * 2.0 < b * 3.0_f64.sqrt() {
        println!("{}", (a * (3.0_f64.sqrt()) * 2.0) / 3.0);
        return;
    }
    let mut lower = b;
    let mut upper = a * (2.0_f64.sqrt());
    while upper - lower > 1e-10 {
        let med = (lower + upper) / 2.0;
        if is_ok(a, b, med) {
            lower = med;
        } else {
            upper = med;
        }
    }
    println!("{}", lower);
}

fn is_ok(a: f64, b: f64, med: f64) -> bool {
    let p = (med * med - a * a).sqrt();
    let q = (med * med - b * b).sqrt();
    med * med < (b - p) * (b - p) + (a - q) * (a - q)
}
