use proconio::input;

fn main() {
    input! {
        h:f64,
    }
    println!("{}", ((12800000.0 + h) * h).sqrt());
}
