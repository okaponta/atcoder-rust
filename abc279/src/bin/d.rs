use proconio::input;

fn main() {
    input! {
        a:f64,
        b:f64,
    }
    let t = (a / (2.0 * b)).powf(2.0 / 3.0);
    let t1 = t.floor();
    let t2 = t.ceil();
    let a1 = t1 * b + a / (t1 + 1.0).sqrt();
    let a2 = t2 * b + a / (t2 + 1.0).sqrt();
    println!("{}", a1.min(a2));
}
