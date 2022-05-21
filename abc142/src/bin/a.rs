use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", ((n + 1) / 2) as f64 / n as f64);
}
