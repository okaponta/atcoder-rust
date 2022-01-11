use proconio::{input, marker::Chars};

fn main() {
    input! {
       s:Chars,
    }
    println!("{}", if s[1] == 'B' { "ARC" } else { "ABC" });
}
