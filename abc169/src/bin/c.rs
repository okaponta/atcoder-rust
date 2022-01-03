use proconio::{input, marker::Chars};

fn main() {
    input! {
       a:i64,b:Chars,
    }
    let mul1 = a * (b[0].to_digit(10).unwrap() as i64);
    let mul2 = a * (b[2].to_digit(10).unwrap() as i64);
    let mul3 = a * (b[3].to_digit(10).unwrap() as i64);
    let ans = (mul3 / 10 + mul2) / 10 + mul1;
    println!("{}", ans);
}
