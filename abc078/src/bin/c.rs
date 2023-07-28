use proconio::input;

fn main() {
    input! {
        n:usize,
        m:usize,
    }
    println!("{}", (m * 1900 + (n - m) * 100) * 1 << m);
}
