use proconio::input;

fn main() {
    input! {
        ab:(i32,i32),
    }
    println!("{}", if ab.0 * ab.1 % 2 == 0 { "No" } else { "Yes" });
}
