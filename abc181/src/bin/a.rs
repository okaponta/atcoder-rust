use proconio::input;

fn main() {
    input! {
       n:i32,
    }
    println!("{}", if n % 2 == 0 { "White" } else { "Black" });
}
