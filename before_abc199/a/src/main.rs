use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    println!("{}", if a * a + b * b < c * c { "Yes" } else { "No" });
}
