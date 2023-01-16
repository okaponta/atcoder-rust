use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
    }
    println!("{}", if n % k == 0 { 0 } else { 1 });
}
