use proconio::input;

fn main() {
    input! {
       a:u64,b:u64,n:u64,
    }
    let x = std::cmp::min(b - 1, n);
    println!("{}", a * x / b);
}
