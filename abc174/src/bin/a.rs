use proconio::input;

fn main() {
    input! {
       x:i32,
    }
    println!("{}", if x >= 30 { "Yes" } else { "No" });
}
