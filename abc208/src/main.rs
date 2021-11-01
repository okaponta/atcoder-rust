use proconio::input;

fn main() {
    input! {
            a: i32,
            b: i32
    }
    println!("{}", if a <= b && b <= 6 * a { "Yes" } else { "No" });
}
