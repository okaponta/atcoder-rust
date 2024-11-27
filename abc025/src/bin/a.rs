use proconio::{marker::*, *};

fn main() {
    input! {s:Chars,n:Usize1}
    println!("{}{}", s[n / 5], s[n % 5]);
}
