use proconio::input;

fn main() {
    input! {
        n:i32,
    }
    println!("{}", if n > 4 || n == 1 { "Yes" } else { "No" });
}
