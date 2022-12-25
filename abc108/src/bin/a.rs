use proconio::input;

fn main() {
    input! {
        k:usize,
    }
    println!("{}", (k / 2) * (k - k / 2));
}
