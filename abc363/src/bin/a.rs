use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    println!("{}", 100 - n % 100);
}
