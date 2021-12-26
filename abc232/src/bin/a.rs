use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,
    }
    let a = s[0].to_digit(10).unwrap();
    let b = s[2].to_digit(10).unwrap();
    println!("{}", a * b);
}
