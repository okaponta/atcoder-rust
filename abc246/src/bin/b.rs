use proconio::input;

fn main() {
    input! {
        a:f64,
        b:f64,
    }
    let dist = (a * a + b * b).sqrt();
    println!("{} {}", a / dist, b / dist);
}
