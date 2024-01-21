use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    let b = (1..s.len()).all(|i| s[i - 1] as u8 <= s[i] as u8);
    println!("{}", if b { "Yes" } else { "No" });
}
