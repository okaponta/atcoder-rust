use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,
    }
    let a = s[0].to_digit(10).unwrap() as i32;
    let b = s[1].to_digit(10).unwrap() as i32;
    let c = s[2].to_digit(10).unwrap() as i32;
    println!("{}", (a + b + c) * 111);
}
