use proconio::input;

fn main() {
    input! {
        n:usize,
    }
    let s = n
        .to_string()
        .chars()
        .fold(0, |s, c| s + c.to_digit(10).unwrap() as usize);
    println!("{}", if n % s == 0 { "Yes" } else { "No" });
}
