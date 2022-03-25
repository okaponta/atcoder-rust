use proconio::input;

fn main() {
    input! {
        n:usize,k:usize,
    }
    println!("{}", (n % k).min(k - n % k));
}
