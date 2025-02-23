use proconio::{marker::*, *};

fn main() {
    input! {
        s:Chars,
    }
    println!(
        "{}{}",
        s[0].to_ascii_uppercase(),
        s[1..].iter().collect::<String>().to_ascii_lowercase()
    );
}
