use proconio::{input, marker::Chars};

fn main() {
    input! {s:Chars}
    let n = s.len();
    let s = (n & 1 < 1) ^ (s[0] == s[n - 1]);
    println!("{}", if s { "Second" } else { "First" });
}
