use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", if n % 2 == 0 { n } else { 2 * n });
}
