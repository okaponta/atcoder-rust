use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!("{}{}{}", s[0], s.len() - 2, s[s.len() - 1]);
}
